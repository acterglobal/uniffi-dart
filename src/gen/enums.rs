use genco::prelude::*;
use uniffi_bindgen::interface::{AsType, Enum};

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
            abstract class $cls_name {}

            $(for variant in obj.variants()
                => class $(class_name(variant.name()))$cls_name extends $cls_name {
                        $(for field in variant.fields() => final $(generate_type(&field.as_type())) $(var_name(field.name()));  )

                        $(class_name(variant.name()))$cls_name(
                            $(for field in variant.fields() => this.$(var_name(field.name())),  )
                        );
                    }
            )
        }

        //TODO!: Generate the lifting and lowering code for each variant
    }
}
