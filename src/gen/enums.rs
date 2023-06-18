use genco::prelude::*;
use uniffi_bindgen::interface::{Enum, Method};

use super::types::{
    convert_from_rust_buffer, convert_to_rust_buffer, generate_ffi_dart_type, generate_ffi_type,
    generate_type, type_lift_fn, type_lower_fn,
};
use super::utils::{class_name, fn_name, var_name};

pub fn generate_enum(obj: &Enum) -> dart::Tokens {
    todo!();
    let cls_name = &class_name(obj.name());
    quote! {
        enum $cls_name {
        }
    }
}
