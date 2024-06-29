use genco::prelude::*;
use uniffi_bindgen::interface::{AsType, Callable, Function};

use crate::gen::oracle::DartCodeOracle;
use crate::gen::render::AsRenderable;

use super::render::TypeHelperRenderer;
use super::utils::{fn_name, var_name};

pub fn generate_function(func: &Function, type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
    let args = quote!($(for arg in &func.arguments() => $(&arg.as_renderable().render_type(&arg.as_type(), type_helper)) $(var_name(arg.name())),));

    let (ret, lifter) = if let Some(ret) = func.return_type() {
        (
            ret.as_renderable().render_type(ret, type_helper),
            quote!($(ret.as_codetype().ffi_converter_name()).lift),
        )
    } else {
        (quote!(void), quote!((_) {}))
    };

    if func.is_async() {
        quote!(
            Future<$ret> $(DartCodeOracle::fn_name(func.name()))($args) {
                return uniffiRustCallAsync(
                  () => _UniffiLib.instance.$(func.ffi_func().name())(
                    $(for arg in &func.arguments() => $(DartCodeOracle::type_lower_fn(&arg.as_type(), quote!($(var_name(arg.name()))))),)
                  ),
                  _UniffiLib.instance.$(func.ffi_rust_future_poll(type_helper.get_ci()).name()),
                  _UniffiLib.instance.$(func.ffi_rust_future_complete(type_helper.get_ci()).name()),
                  _UniffiLib.instance.$(func.ffi_rust_future_free(type_helper.get_ci()).name()),
                  $lifter,
                );
            }
        )
    } else {
        quote!(
            $ret $(DartCodeOracle::fn_name(func.name()))($args) {
                return rustCall((status) => $lifter(_UniffiLib.instance.$(func.ffi_func().name())(
                    $(for arg in &func.arguments() => $(DartCodeOracle::type_lower_fn(&arg.as_type(), quote!($(var_name(arg.name()))))),) status
                )));
            }
        )
    }
}

