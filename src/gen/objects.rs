use genco::prelude::*;
use uniffi_bindgen::interface::{Method, Object};

use super::types::{
    generate_ffi_dart_type, generate_ffi_type, generate_type, type_lift_fn, type_lower_fn,
};
use super::utils::{class_name, fn_name, var_name};

pub fn generate_object(obj: &Object) -> dart::Tokens {
    let cls_name = &class_name(obj.name());
    quote! {
        class $cls_name {
            final Api _api;
            final Pointer<Void> _ptr;

            $(cls_name)._(this._api, this._ptr);

            factory $(cls_name).lift(Api api, Pointer<Void> ptr) {
                return $(cls_name)._(api, ptr);
            }

            void drop() {
                final _freePtr = _api._lookup<
                NativeFunction<
                    Void Function(Pointer<Void>, Pointer<RustCallStatus>)>>($(format!("\"{}\"", obj.ffi_object_free().name())));
                final free = _freePtr.asFunction<void Function(Pointer<Void>, Pointer<RustCallStatus>)>();
                rustCall(_api, (res) => free(_ptr, res));
            }

            $(for mt in &obj.methods() => $(generate_method(mt)))
        }
    }
}

#[allow(unused_variables)]
pub fn generate_method(fun: &Method) -> dart::Tokens {
    let api = "_api";
    let ffi = fun.ffi_func();
    let fn_name = fn_name(fun.name());
    let args = quote!($(for arg in &fun.arguments() => $(generate_type(arg.type_())) $(var_name(arg.name())),));
    let ff_name = ffi.name();
    let inner = quote! {
    rustCall(_api, (res) =>
        _$(&fn_name)(
            _ptr,
            $(for arg in &fun.arguments() => $(type_lower_fn(arg.type_(), quote!($(var_name(arg.name()))))),)
        res)
    )
    };

    let (ret, body) = if let Some(ret) = fun.return_type() {
        (
            generate_type(ret),
            quote! {
                return $(type_lift_fn(ret, inner));
            },
        )
    } else {
        (quote!(void), quote!($inner;))
    };

    quote! {
        late final _$(&fn_name)Ptr = _api._lookup<
        NativeFunction<
            $(generate_ffi_type(ffi.return_type())) Function(
                $(for arg in &ffi.arguments() => $(generate_ffi_type(Some(&arg.type_()))),)
                Pointer<RustCallStatus>
        )>>($(format!("\"{ff_name}\"")));

        late final _$(&fn_name) = _$(&fn_name)Ptr.asFunction<
        $(generate_ffi_dart_type(ffi.return_type())) Function(
            $(for arg in &ffi.arguments() => $(generate_ffi_dart_type(Some(&arg.type_()))),)
            Pointer<RustCallStatus>
        )>();

        $ret $fn_name ($args) {
            $body
        }
    }
}
