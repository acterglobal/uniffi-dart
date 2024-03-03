use genco::prelude::*;
use uniffi_bindgen::interface::{AsType, Type, Enum, Variant, Field};

use super::types::{
    convert_from_rust_buffer, convert_to_rust_buffer, generate_ffi_dart_type, generate_ffi_type,
    generate_type, type_lift_fn, type_lower_fn,
};

use super::utils::{class_name, enum_variant_name, var_name};

pub fn generate_enum(obj: &Enum) -> dart::Tokens {
    let cls_name = &class_name(obj.name());
    if obj.is_flat() {
        quote! {
            enum $cls_name {
                $(for variant in obj.variants() => $(enum_variant_name(variant.name())),);

                factory $cls_name.lift(Api api, RustBuffer buffer) {
                    final index = buffer.toIntList().buffer.asByteData().getInt32(0);
                    $(for (index, variant) in obj.variants().iter().enumerate() =>
                        if (index == $(index+1)) {
                            return $cls_name.$(enum_variant_name(variant.name()));
                        }
                    )
                    throw UniffiInternalError(UniffiInternalError.unexpectedEnumCase, "Unable to determine enum variant");
                }
    
                static Uint8List lower(Api api, $cls_name input) {
                    return createUint8ListFromInt(input.index + 1); // So enums aren't zero indexed?
                }
            }
        }
    } else {
        quote! {
            abstract class $cls_name {
                $cls_name();

                factory $cls_name.lift(Api api, RustBuffer buffer) {
                    final index = buffer.toIntList().buffer.asByteData().getInt32(0);
                    // Pass lifting onto the appropriate variant. based on index...variants are not 0 index
                    $(for (index, variant) in obj.variants().iter().enumerate() =>
                        if (index == $(index+1)) {
                            return $(variant.name())$cls_name.lift(api, buffer);
                        }
                    )
                    // If no return happens
                    throw UniffiInternalError(UniffiInternalError.unexpectedEnumCase, "Unable to determine enum variant");
                    // return $(class_name(obj.variants()[6].name()))Value(6);
                    // //return $cls_name(7);
                }
                
                static Uint8List lower(Api api, Value value) {
                    // Each variant has a lower method, simply pass on it's return
                    $(for (_index, variant) in obj.variants().iter().enumerate() =>
                        if (value is $(variant.name())$cls_name) {
                            return value.lower(api);
                        }
                    )
                    throw UniffiInternalError(UniffiInternalError.unexpectedEnumCase, "Unable to determine enum variant to lower");
                }
            }

            $(for (index, variant) in obj.variants().iter().enumerate()
                => class $(class_name(variant.name()))$cls_name extends $cls_name {
                        $(for field in variant.fields() => final $(generate_type(&field.as_type())) $(var_name(field.name()));  )

                        $(class_name(variant.name()))$cls_name($(for field in variant.fields() => this.$(var_name(field.name())),  ));

                        $(generate_variant_factory(cls_name, variant))

                        $(generate_variant_lowerer(cls_name, index, variant))
                    }
            )
        }
        //TODO!: Generate the lowering code for each variant
    }
}

fn generate_variant_factory(cls_name: &String, variant: &Variant) -> dart::Tokens {
    //
    fn generate_variant_field_lifter(field: &Field, uint8_list_var: &dart::Tokens, results_list: dart::Tokens, index: usize, offset_var: &dart::Tokens) -> dart::Tokens {
        match field.as_type() {
             Type::Int8 | Type::UInt8 => quote!($results_list.insert($index, liftInt8OrUint8($uint8_list_var, $offset_var)); $offset_var += 1; ),
             Type::Int16 | Type::UInt16 => quote!($results_list.insert($index, liftInt16OrUint16($uint8_list_var, $offset_var)); $offset_var += 2; ),
             Type::Int32 | Type::UInt32 => quote!($results_list.insert($index, liftInt32OrUint32($uint8_list_var, $offset_var)); $offset_var += 4; ),
             Type::Int64 | Type::UInt64 => quote!($results_list.insert($index, liftInt64OrUint64($uint8_list_var, $offset_var)); $offset_var += 8; ),
             Type::Float32 => quote!($results_list.insert($index, liftFloat32($uint8_list_var, $offset_var)); $offset_var += 4; ),
             Type::Float64 => quote!($results_list.insert($index, liftFloat64($uint8_list_var, $offset_var)); $offset_var += 1;  ) ,
             Type::Boolean => quote!($results_list.insert($index, liftBoolean($uint8_list_var, $offset_var)); $offset_var += 1;  ),
             Type::String => quote!(final v = liftVaraibleLength($uint8_list_var, (buf) => liftString(api, buf), $offset_var);  $results_list.insert($index, v.data); $offset_var += v.offset;),
             Type::Enum ( name ) => quote! {
                print($uint8_list_var);
                $results_list.insert($index, liftBoolean($uint8_list_var, $offset_var)); $offset_var += 1;
             },
             Type::Sequence ( inner_type ) => {
                let element_size =  match inner_type.as_type() {
                    Type::UInt8 |  Type::Int8 | Type::Boolean => 1,
                    Type::UInt16 | Type::Int16 => 2,
                    Type::UInt32 | Type::Int32 | Type::Float32=> 4,
                    Type::UInt64 | Type::Int64 | Type::Float64=> 8,
                    // Values lower than zero have variable lengths
                    Type::String => -1, // Expect a string
                    _ => todo!("Sequence containing {:?}, not implemented", inner_type)
                };
                quote! {
               
                final data = liftSequence(api, $uint8_list_var, liftInt8OrUint8, $element_size, $offset_var);  
                $results_list.insert($index, data); 
                //$offset_var += v.offset;
             
             }},
             _ => todo!("offset/size of Type::{:?}", field.as_type())
         }
    }

    quote! {
        factory $(class_name(variant.name()))$cls_name.lift(Api api, RustBuffer buffer) {
            Uint8List input = buffer.toIntList();
            int offset = 4; // Start at 4, because the first 32bits are the enum index
            List<dynamic> results = [];

            $(for (index, field) in variant.fields().iter().enumerate() => $(generate_variant_field_lifter(field, &quote!(input), quote!(results), index, &quote!(offset), )))

            return $(class_name(variant.name()))$cls_name($( for (index, _) in variant.fields().iter().enumerate() => results[$(index)], ));
        }
    }
}

fn generate_variant_lowerer(_cls_name: &String, index: usize, variant: &Variant) -> dart::Tokens {
    fn generate_variant_field_lowerer(field: &Field, _index: usize, offset_var: &dart::Tokens) -> dart::Tokens {
        // TODO:  other enums, maps, and vectors etc...
        let lowerer = match field.as_type() {
            Type::Int8 => quote!(lowerInt8(this.$(field.name()))),
            Type::UInt8 => quote!(lowerUint8(api, this.$(field.name()))),
            Type::Int16 => quote!(lowerInt16(this.$(field.name()))),
            Type::UInt16 => quote!(lowerUint16(this.$(field.name()))),
            Type::Int32 | Type::UInt32 | Type::Int64 | Type::UInt64  => quote!(createUint8ListFromInt(this.$(field.name()))),
            Type::Float32 => quote!(lowerFloat32(this.$(field.name()))),
            Type::Float64  => quote!(lowerFloat64(this.$(field.name()))),
            Type::Boolean => quote!(Uint8List.fromList([this.$(field.name()) ? 1 : 0])),
            Type::String => quote!(lowerVaraibleLength(api,this.$(field.name()), lowerString) ),
            Type::Sequence ( inner_type ) => quote!(lowerSequence(api, this.$(field.name()), lowerUint8, 1)),
            _ => todo!("Add variant field lifter for type: {:?}", field.as_type())
        };

        quote! {
            //throw UnimplementedError("Create a list for all the different types, strings, bools, other enums, etc");
            final $(field.name()) = $(lowerer);  
            $offset_var += $(field.name()).length;
        }
    }

    quote! {
        Uint8List lower(Api api) {
            // Turn all the fields to their int lists reprsentations
            final index = createUint8ListFromInt($(index + 1));
            int offset = 0;
            offset += index.length;
            $(for (index, field) in variant.fields().iter().enumerate() => $(generate_variant_field_lowerer(field, index, &quote!(offset))))
            // Create a list with big enough for all the fields and reset the offset
            final res = Uint8List(offset);
            offset = 0;
            // First set the index
            res.setAll(offset, index);
            offset += index.length;
            // Now set the rest of the fields
            $(for field in variant.fields() => 
                res.setAll(offset, $(field.name()));
                offset += $(field.name()).length;
            )

            res.setAll(index.length, value);
            return res;
        }
    }
}