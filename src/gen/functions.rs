use genco::prelude::*;
use uniffi_bindgen::interface::Function;

use super::types::{
    generate_ffi_dart_type, generate_ffi_type, generate_type, generate_type_lift_fn,
};
use super::utils::{fn_name, var_name};

#[allow(unused_variables)]
pub fn generate_function(fun: &Function) -> dart::Tokens {
    let ffi = fun.ffi_func();
    let fn_name = fn_name(fun.name());
    let args = quote!($(for arg in &fun.arguments() => $(generate_type(arg.type_())) $(var_name(arg.name())),));
    let (ret, ret_lift) = if let Some(ret) = fun.return_type() {
        (generate_type(ret), generate_type_lift_fn(ret))
    } else {
        (quote!(void), None::<Tokens<Dart>>)
    };
    let ff_name = ffi.name();
    let inner = quote!(
        _$(&fn_name)(
        $(for arg in &fun.arguments() => $(var_name(arg.name())),)
    ));

    let body = if let Some(lift_fn) = ret_lift {
        quote! {
            return $lift_fn($inner);
        }
    } else {
        quote! {
            return $inner;
        }
    };

    quote! {
        late final _$(&fn_name)Ptr = _lookup<
        ffi.NativeFunction<
            $(generate_ffi_type(ffi.return_type())) Function(
                $(for arg in &ffi.arguments() => $(generate_ffi_type(Some(&arg.type_()))),)
        )>>($(format!("\"{ff_name}\"")));

        late final _$(&fn_name) = _$(&fn_name)Ptr.asFunction<
        $(generate_ffi_dart_type(ffi.return_type())) Function(
            $(for arg in &ffi.arguments() => $(generate_ffi_dart_type(Some(&arg.type_()))),)
        )>();

        $ret $fn_name ($args) {
            $body
        }
    }
}
