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

                        factory $(class_name(variant.name()))$cls_name.lift(Api api, RustBuffer buffer) {
                            // TODO: Impliment factory builders for each variant
                            
                            throw UniffiInternalError(6, "Not implimented");
                        }
                    }
            )
        }

        //TODO!: Generate the lifting and lowering code for each variant
    }
}
