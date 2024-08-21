use genco::prelude::*;
use uniffi_bindgen::interface::{AsType, Function};

use crate::gen::oracle::DartCodeOracle;
use crate::gen::render::AsRenderable;

use super::oracle::AsCodeType;
use super::render::TypeHelperRenderer;

// #[allow(unused_variables)]
// pub fn generate_function(
//     api: &str,
//     fun: &Function,
//     type_helper: &dyn TypeHelperRenderer,
// ) -> dart::Tokens {
//     let ffi = fun.ffi_func();
//     let fn_name = fn_name(fun.name());
//     let args = quote!($(for arg in &fun.arguments() => $(&arg.as_renderable().render_type(&arg.as_type(), type_helper)) $(DartCodeOracle::var_name(arg.name())),));
//     let ff_name = ffi.name();
//     let inner = quote! {
//     rustCall((res) =>
//         _$(&fn_name)(
//             $(for arg in &fun.arguments() => $(DartCodeOracle::type_lower_fn(&arg.as_type(), quote!($(DartCodeOracle::var_name(arg.name()))))),)
//         res)
//     )
//     };

//     let (ret, body) = if let Some(ret) = fun.return_type() {
//         (
//             ret.as_renderable().render_type(ret, type_helper),
//             quote! {
//                 return $(DartCodeOracle::type_lift_fn(ret, inner));
//             },
//         )
//     } else {
//         (quote!(void), quote!($inner;))
//     };

//     quote! {
//         late final _$(&fn_name)Ptr = _lookup<
//         NativeFunction<
//             $(DartCodeOracle::ffi_native_type_label(ffi.return_type())) Function(
//                 $(for arg in &ffi.arguments() => $(DartCodeOracle::ffi_native_type_label(Some(&arg.type_()))),)
//                 Pointer<RustCallStatus>
//         )>>($(format!("\"{ff_name}\"")));

//         late final _$(&fn_name) = _$(&fn_name)Ptr.asFunction<
//         $(DartCodeOracle::ffi_dart_type_label(ffi.return_type())) Function(
//             $(for arg in &ffi.arguments() => $(DartCodeOracle::ffi_dart_type_label(Some(&arg.type_()))),)
//             Pointer<RustCallStatus>
//         )>();

//         $ret $fn_name ($args) {
//             final api = $api;
//             $body
//         }
//     }
// }

// #[allow(unused_variables)]
// pub fn generate_function(
//     api: &str,
//     fun: &Function,
//     type_helper: &dyn TypeHelperRenderer,
// ) -> dart::Tokens {
//     let ffi = fun.ffi_func();
//     let fn_name = fn_name(fun.name());
//     let args = quote!($(for arg in &fun.arguments() => $(&arg.as_renderable().render_type(&arg.as_type(), type_helper)) $(DartCodeOracle::var_name(arg.name())),));
//     let ff_name = ffi.name();
//     let inner = quote! {
//     rustCall((res) =>
//         _$(&fn_name)(
//             $(for arg in &fun.arguments() => $(DartCodeOracle::type_lower_fn(&arg.as_type(), quote!($(DartCodeOracle::var_name(arg.name()))))),)
//         res)
//     )
//     };

//     let (ret, body) = if let Some(ret) = fun.return_type() {
//         (
//             ret.as_renderable().render_type(ret, type_helper),
//             quote! {
//                 return $(DartCodeOracle::type_lift_fn(ret, inner));
//             },
//         )
//     } else {
//         (quote!(void), quote!($inner;))
//     };

//     quote! {
//         late final _$(&fn_name)Ptr = _lookup<
//         NativeFunction<
//             $(DartCodeOracle::ffi_native_type_label(ffi.return_type())) Function(
//                 $(for arg in &ffi.arguments() => $(DartCodeOracle::ffi_native_type_label(Some(&arg.type_()))),)
//                 Pointer<RustCallStatus>
//         )>>($(format!("\"{ff_name}\"")));

//         late final _$(&fn_name) = _$(&fn_name)Ptr.asFunction<
//         $(DartCodeOracle::ffi_dart_type_label(ffi.return_type())) Function(
//             $(for arg in &ffi.arguments() => $(DartCodeOracle::ffi_dart_type_label(Some(&arg.type_()))),)
//             Pointer<RustCallStatus>
//         )>();

//         $ret $fn_name ($args) {
//             final api = $api;
//             $body
//         }
//     }
// }

pub fn generate_function(func: &Function, type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
    // if func.takes_self() {} // TODO: Do something about this condition
    let args = quote!($(for arg in &func.arguments() => $(&arg.as_renderable().render_type(&arg.as_type(), type_helper)) $(DartCodeOracle::var_name(arg.name())),));

    let (ret, lifter) = if let Some(ret) = func.return_type() {
        (
            ret.as_renderable().render_type(ret, type_helper),
            quote!($(ret.as_codetype().lift())),
        )
    } else {
        (quote!(void), quote!((_) {}))
    };

    if func.is_async() {
        quote!(
            Future<$ret> $(DartCodeOracle::fn_name(func.name()))($args) {
                return uniffiRustCallAsync(
                  () => $(DartCodeOracle::find_lib_instance()).$(func.ffi_func().name())(
                    $(for arg in &func.arguments() => $(DartCodeOracle::type_lower_fn(&arg.as_type(), quote!($(DartCodeOracle::var_name(arg.name()))))),)
                  ),
                  $(DartCodeOracle::async_poll(func, type_helper.get_ci())),
                  $(DartCodeOracle::async_complete(func, type_helper.get_ci())),
                  $(DartCodeOracle::async_free(func, type_helper.get_ci())),
                  $lifter,
                );
            }

        )
    } else {
        quote!(
            $ret $(DartCodeOracle::fn_name(func.name()))($args) {
                return rustCall((status) => $lifter($(DartCodeOracle::find_lib_instance()).$(func.ffi_func().name())(
                    $(for arg in &func.arguments() => $(DartCodeOracle::type_lower_fn(&arg.as_type(), quote!($(DartCodeOracle::var_name(arg.name()))))),) status
                )));
            }
        )
    }
}
