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
                $(for variant in obj.variants() => $(enum_variant_name(variant.name())),)
            }
        }
    } else {
        quote! {
            abstract class $cls_name {
                $cls_name();

                factory Value.lift(Api api, RustBuffer buffer) {
                    final index = buffer.toIntList().buffer.asByteData().getInt32(0);
                    // Pass lifting onto the appropriate variant. based on index...variants are not 0 index
                    $(for (index, variant) in obj.variants().iter().enumerate() =>
                        if (index == $(index+1)) {
                            return $(variant.name())$cls_name.lift(api, buffer);
                        }
                    )
                    // If no return happens
                    throw UniffiInternalError(6, "Unable to determine enum variant");
                    // return $(class_name(obj.variants()[6].name()))Value(6);
                    // //return $cls_name(7);
                  }
            }

            $(for variant in obj.variants()
                => class $(class_name(variant.name()))$cls_name extends $cls_name {
                        $(for field in variant.fields() => final $(generate_type(&field.as_type())) $(var_name(field.name()));  )

                        $(class_name(variant.name()))$cls_name($(for field in variant.fields() => this.$(var_name(field.name())),  ));

                        $(generate_variant_factory(cls_name, variant))
                    }
            )
        }
        //TODO!: Generate the lowering code for each variant
    }
}

fn generate_variant_factory(cls_name: &String, variant: &Variant) -> dart::Tokens {
    //
    fn generate_variant_field_lifter(field: &Field, uint8_list_var: dart::Tokens, results_list: dart::Tokens, index: usize, offset_var: &dart::Tokens) -> dart::Tokens {
        match field.as_type() {
             Type::Int8 | Type::UInt8 => quote!($results_list.insert($index, liftInt8OrUint8($uint8_list_var, $offset_var)); $offset_var += 1; ),
             Type::Int16 | Type::UInt16 => quote!($results_list.insert($index, liftInt16OrUint16($uint8_list_var, $offset_var)); $offset_var += 2; ),
             Type::Int32 | Type::UInt32 => quote!($results_list.insert($index, liftInt32OrUint32($uint8_list_var, $offset_var)); $offset_var += 4; ),
             Type::Int64 | Type::UInt64 => quote!($results_list.insert($index, liftInt64OrUint64($uint8_list_var, $offset_var)); $offset_var += 8; ),
             Type::Float32 => quote!($results_list.insert($index, liftFloat32($uint8_list_var, $offset_var)); $offset_var += 4; ),
             Type::Float64 => quote!($results_list.insert($index, liftFloat64($uint8_list_var, $offset_var)); $offset_var += 1;  ) ,
             Type::Boolean => quote!($results_list.insert($index, liftBoolean($uint8_list_var, $offset_var)); $offset_var += 1;  ),
             Type::String => quote!(final v = liftVaraibleLength($uint8_list_var, (buf) => liftString(api, buf), $offset_var);  $results_list.insert($index, v.data); $offset_var += v.offset;),
             _ => todo!("offset/size of Type::{:?}", field.as_type())
         }
    }

    quote! {
        factory $(class_name(variant.name()))$cls_name.lift(Api api, RustBuffer buffer) {
            Uint8List input = buffer.toIntList();
            
            int offset = 4; // Start at 4, because the first 32bits are the enum index
            List<dynamic> results = [];

            $(for (index, field) in variant.fields().iter().enumerate() => $(generate_variant_field_lifter(field, quote!(input), quote!(results), index, &quote!(offset), )))

            return $(class_name(variant.name()))$cls_name($( for (index, _) in variant.fields().iter().enumerate() => results[$(index)], ));
        }
    }
}

// TODO!: Generate the lowring code