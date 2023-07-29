use genco::prelude::*;
use uniffi_bindgen::interface::Enum;

use super::utils::{class_name, enum_variant_name};

pub fn generate_enum(obj: &Enum) -> dart::Tokens {
    let cls_name = &class_name(obj.name());
    if obj.is_flat() {
        let variants =
            quote!($(for variant in obj.variants() => $(enum_variant_name(variant.name())),));
        quote! {
            enum $cls_name {
                $variants
            }
        }
    } else {
        todo!("Add enum complex accociated types")
    }
}
