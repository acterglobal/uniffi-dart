use genco::prelude::*;
use uniffi_bindgen::backend::{CodeType, Literal};
use uniffi_bindgen::interface::{AsType, Type, Enum, Variant, Field};

use super::oracle::{DartCodeOracle, AsCodeType};
use super::render::{Renderable, AsRenderable, TypeHelperRenderer};
use super::types::{
    convert_from_rust_buffer, convert_to_rust_buffer, generate_ffi_dart_type, generate_ffi_type,
    generate_type, type_lift_fn, type_lower_fn,
};

use super::utils::{class_name, enum_variant_name, var_name};

#[derive(Debug)]
pub struct EnumCodeType {
    id: String,
}

impl EnumCodeType {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

impl CodeType for EnumCodeType {
    fn type_label(&self) -> String {
        DartCodeOracle::class_name(&self.id)
    }

    fn canonical_name(&self) -> String {
        format!("{}", self.id)
    }

    fn literal(&self, literal: &Literal) -> String {
        if let Literal::Enum(v, _) = literal {
            format!(
                "{}{}",
                self.type_label(),
                DartCodeOracle::enum_variant_name(v)
            )
        } else {
            unreachable!();
        }
    }

    fn ffi_converter_name(&self) -> String {
        format!("{}", self.canonical_name()) // Objects will use factory methods
    }
}

impl Renderable for EnumCodeType {
    fn render_type_helper(&self, type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
        if type_helper.check(&self.id) {
            quote!()
        } else {
            if let Some(enum_) = type_helper.get_enum(&self.id) {
                generate_enum(enum_, type_helper)
            } else {
                unreachable!()
            }
        }
    }
}


pub fn generate_enum(obj: &Enum, type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
    let cls_name = &DartCodeOracle::class_name(obj.name());
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
    
                static RustBuffer lower(Api api, $cls_name input) {
                    return toRustBuffer(api, createUint8ListFromInt(input.index + 1)); // So enums aren't zero indexed?
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
                
                static RustBuffer lower(Api api, Value value) {
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
                => class $(DartCodeOracle::class_name(variant.name()))$cls_name extends $cls_name {
                    // TODO: Replace render type with with shorter method, ideally provided by DartCodeOracle
                        $(for field in variant.fields() => final $(&field.as_type().as_renderable().render_type(&field.as_type(), type_helper)) $(var_name(field.name()));  )

                        $(DartCodeOracle::class_name(variant.name()))$cls_name($(for field in variant.fields() => this.$(var_name(field.name())),  ));

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
    fn generate_variant_field_lifter(field: &Field, input_list: &dart::Tokens, results_list: dart::Tokens, index: usize, offset_var: &dart::Tokens) -> dart::Tokens {
        if let Type::Sequence(_) = field.as_type() {
            return quote!(
                $results_list.insert($index, $(field.as_type().as_codetype().lift())(api, buffer, $offset_var));
                $offset_var += $(field.as_type().as_codetype().ffi_converter_name())().allocationSize($input_list);
            )
        }
        
        if Type::Boolean == field.as_type() {
            quote!(
                $results_list.insert($index, $(field.as_type().as_codetype().lift())( api, $input_list[$offset_var] ));
                $offset_var += $(field.as_type().as_codetype().ffi_converter_name())().allocationSize();
            )
        } else if Type::String == field.as_type() {
            quote!(
                $results_list.insert($index, $(field.as_type().as_codetype().lift())(api,buffer, $offset_var+4));
                $offset_var += $(field.as_type().as_codetype().ffi_converter_name())().allocationSize();
            )          
        } else {
            quote!(
                $results_list.insert($index, $(field.as_type().as_codetype().lift())(api, buffer, $offset_var));
                $offset_var += $(field.as_type().as_codetype().ffi_converter_name())().allocationSize();
            )
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
    fn generate_variant_field_lower(field: &Field, _index: usize, offset_var: &dart::Tokens) -> dart::Tokens {
        let lower_fn = quote!($(field.as_type().as_codetype().lower())(api, this.$(field.name())));

        quote! {
            final $(field.name()) = $(lower_fn);  
            $offset_var += $(field.as_type().as_codetype().ffi_converter_name())().allocationSize(this.$(field.name()));
        }
    }

    quote! {
        RustBuffer lower(Api api) {
            // Turn all the fields to their int lists representations
            final index = createUint8ListFromInt($(index + 1));
            int offset = 0;
            offset += index.length;
            $(for (index, field) in variant.fields().iter().enumerate() => $(generate_variant_field_lower(field, index, &quote!(offset))))
            // Create a list with big enough for all the fields and reset the offset
            final res = Uint8List(offset);
            offset = 0;
            // First set the index
            res.setAll(offset, index);
            offset += index.length;
            // Now set the rest of the fields
            $(for field in variant.fields() => 
                $(match field.as_type() {
                    Type::Boolean => 
                        res.setAll(offset, Uint8List.fromList([$(field.name())]));
                        offset += $(field.as_type().as_codetype().ffi_converter_name())().allocationSize();
                    ,
                    Type::String => 
                        res.setAll(offset, createUint8ListFromInt(this.$(field.name()).length));
                        offset += 4;
                        res.setAll(offset, Uint8List.fromList($(field.name()).toIntList()));
                        offset += $(field.as_type().as_codetype().ffi_converter_name())().allocationSize(this.$(field.name()));
                    ,
                    _ => 
                        res.setAll(offset, $(field.name()).toIntList());
                        offset += $(field.as_type().as_codetype().ffi_converter_name())().allocationSize(this.$(field.name()));  
                    ,
                })
            )

            return toRustBuffer(api, res);
        }
    }
}