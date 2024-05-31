use genco::prelude::*;
use uniffi_bindgen::backend::Type;
use uniffi_bindgen::interface::{AsType, Callable, ExternalKind, FfiFunction, Function};

use crate::gen::oracle::DartCodeOracle;
use crate::gen::render::AsRenderable;

use super::render::TypeHelperRenderer;
use super::utils::{fn_name, var_name};

pub fn generate_function(
    api: &str,
    fun: &Function,
    type_helper: &dyn TypeHelperRenderer,
) -> dart::Tokens {
    generate_for_callable(api, type_helper, fun, fn_name(fun.name()), fun.ffi_func())
}

pub fn generate_for_callable(
    api: &str,
    type_helper: &dyn TypeHelperRenderer,
    fun: &impl Callable,
    fn_name: String,
    ffi: &FfiFunction,
) -> dart::Tokens {
    let with_self = if fun.takes_self() {
        quote!(uniffiClonePointer(),)
    } else {
        quote!()
    };

    let call_signature = quote!($fn_name($(for arg in &fun.arguments() => $(
        &arg.as_renderable().render_type(&arg.as_type(), type_helper)) $(var_name(arg.name())),)));

    if fun.is_async() {
        generate_for_callable_async(api, type_helper, fun, call_signature, ffi.name(), with_self)
    } else {
        generate_for_callable_sync(
            api,
            type_helper,
            fun,
            call_signature,
            ffi.name(),
            with_self,
            ffi.has_rust_call_status_arg(),
        )
    }
}

fn generate_for_callable_async(
    api: &str,
    type_helper: &dyn TypeHelperRenderer,
    fun: &impl Callable,
    fn_signature: dart::Tokens,
    ffi_name: &str,
    with_self: dart::Tokens,
) -> dart::Tokens {
    let (ret, body) = if let Some(ret) = fun.return_type() {
        (
            ret.as_renderable().render_type(&ret, type_helper),
            quote! {
                return $(DartCodeOracle::type_lift_fn(&ret, quote!(status)));
            },
        )
    } else {
        (quote!(void), quote!(return null;))
    };
    let ci = type_helper.get_ci();

    let async_complete = match fun.return_type() {
        Some(Type::External {
            kind: ExternalKind::DataClass,
            name: _,
            ..
        }) => {
            todo!("Need to convert the RustBuffer from our package to the RustBuffer of the external package")
        }
        _ => quote!($(DartCodeOracle::find_lib_instance()).$(fun.ffi_rust_future_complete(ci))),
    };
    quote!(
        Future<$ret> $(fn_signature) {
            final api = $api;
            return uniffiRustCallAsync(
              () => $(DartCodeOracle::find_lib_instance()).$(ffi_name)(
                $(with_self)
                $(for arg in &fun.arguments() => $(DartCodeOracle::type_lower_fn(&arg.as_type(), quote!($(var_name(arg.name()))))),)
              ),
              $(DartCodeOracle::find_lib_instance()).$(fun.ffi_rust_future_poll(ci)),
              $(async_complete),
              $(DartCodeOracle::find_lib_instance()).$(fun.ffi_rust_future_free(ci)),
              (status) {
                $body
              }
            );
        }
    )
}

fn generate_for_callable_sync(
    api: &str,
    type_helper: &dyn TypeHelperRenderer,
    fun: &impl Callable,
    fn_signature: dart::Tokens,
    ffi_name: &str,
    with_self: dart::Tokens,
    has_rust_call_status_arg: bool,
) -> dart::Tokens {
    let inner = if has_rust_call_status_arg {
        quote! {
            rustCall((status) =>
                $(DartCodeOracle::find_lib_instance()).$(ffi_name)(
                    $(with_self)
                    $(for arg in &fun.arguments() => $(DartCodeOracle::type_lower_fn(&arg.as_type(), quote!($(var_name(arg.name()))))),)
                status)
            )
        }
    } else {
        quote! {
            () => $(DartCodeOracle::find_lib_instance()).$(ffi_name)(
                $(with_self)
                $(for arg in &fun.arguments() => $(DartCodeOracle::type_lower_fn(&arg.as_type(), quote!($(var_name(arg.name()))))),)
            )
        }
    };

    let (ret, body) = if let Some(ret) = fun.return_type() {
        (
            ret.as_renderable().render_type(&ret, type_helper),
            quote! {
                return $(DartCodeOracle::type_lift_fn(&ret, inner));
            },
        )
    } else {
        (quote!(void), quote!(return;))
    };
    quote! {
            $ret $(fn_signature) {
                final api = $api;
                $body
            }
    }
}
