use genco::prelude::*;
use uniffi_bindgen::interface::{AsType, Function};

use crate::gen::oracle::DartCodeOracle;
use crate::gen::render::AsRenderable;

use super::render::TypeHelperRenderer;
use super::utils::{fn_name, var_name};

#[allow(unused_variables)]
pub fn generate_function(
    api: &str,
    fun: &Function,
    type_helper: &dyn TypeHelperRenderer,
) -> dart::Tokens {
    let ffi = fun.ffi_func();
    let fn_name = fn_name(fun.name());
    let args = quote!($(for arg in &fun.arguments() => $(&arg.as_renderable().render_type(&arg.as_type(), type_helper)) $(var_name(arg.name())),));
    let ff_name = ffi.name();

    if fun.is_async() {
        let (ret, body) = if let Some(ret) = fun.return_type() {
            (
                ret.as_renderable().render_type(ret, type_helper),
                quote! {
                    return $(DartCodeOracle::type_lift_fn(ret, quote!(res)));
                },
            )
        } else {
            (quote!(void), quote!(return null;))
        };
        quote!(
            Future<$ret> $(DartCodeOracle::fn_name(fun.name()))($args) {
                final api = $api;
                return uniffiRustCallAsync(
                  () => $(DartCodeOracle::find_lib_instance()).$(fun.ffi_func().name())(
                    $(for arg in &fun.arguments() => $(DartCodeOracle::type_lower_fn(&arg.as_type(), quote!($(var_name(arg.name()))))),)
                  ),
                  $(DartCodeOracle::async_poll(fun, type_helper.get_ci())),
                  $(DartCodeOracle::async_complete(fun, type_helper.get_ci())),
                  $(DartCodeOracle::async_free(fun, type_helper.get_ci())),
                  (res) {
                    final api = $api;
                    $body
                  }
                );
            }
        )
    } else {
        let inner = quote! {
            rustCall(api, (res) =>
                $(DartCodeOracle::find_lib_instance()).$(fun.ffi_func().name())(
                    $(for arg in &fun.arguments() => $(DartCodeOracle::type_lower_fn(&arg.as_type(), quote!($(var_name(arg.name()))))),)
                res)
            )
        };

        let (ret, body) = if let Some(ret) = fun.return_type() {
            (
                ret.as_renderable().render_type(ret, type_helper),
                quote! {
                    return $(DartCodeOracle::type_lift_fn(ret, inner));
                },
            )
        } else {
            (quote!(void), quote!(return;))
        };
        quote! {
                $ret $fn_name ($args) {
                    final api = $api;
                    $body
                }
        }
    }
}
