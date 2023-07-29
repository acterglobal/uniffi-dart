use genco::prelude::*;
use uniffi_bindgen::interface::{AsType, Method, Record};

use super::types::{
    convert_from_rust_buffer, convert_to_rust_buffer, generate_ffi_dart_type, generate_ffi_type,
    generate_type, type_lift_fn, type_lower_fn,
};
use super::utils::{class_name, fn_name, var_name};

pub fn generate_record(obj: &Record) -> dart::Tokens {
    let cls_name = &class_name(obj.name());
    quote! {
        class $cls_name {
            $(for f in obj.fields() => final $(generate_type(&f.as_type())) $(var_name(f.name()));)

            $(cls_name)._($(for f in obj.fields() => this.$(var_name(f.name())), ));

            // factory $(cls_name).lift(Api api, Pointer<Void> ptr) {
            //     return $(cls_name)._(api, ptr);
            // }

        }
    }
}
