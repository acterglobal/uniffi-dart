use genco::prelude::*;
use uniffi_bindgen::backend::{CodeType, Literal};
use uniffi_bindgen::interface::{AsType, Method, Object};

use crate::gen::oracle::DartCodeOracle;
use crate::gen::render::AsRenderable;
use crate::gen::render::{Renderable, TypeHelperRenderer};

use super::utils::{class_name, fn_name, var_name};

#[derive(Debug)]
pub struct ObjectCodeType {
    id: String,
}

impl ObjectCodeType {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

impl CodeType for ObjectCodeType {
    fn type_label(&self) -> String {
        DartCodeOracle::class_name(&self.id)
    }

    fn canonical_name(&self) -> String {
        self.id.to_string()
    }

    fn literal(&self, _literal: &Literal) -> String {
        unreachable!();
    }

    fn ffi_converter_name(&self) -> String {
        format!("FfiConverter{}", self.canonical_name())
    }
}

impl Renderable for ObjectCodeType {
    fn render_type_helper(&self, type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
        if type_helper.check(&self.id) {
            quote!()
        } else if let Some(obj) = type_helper.get_object(&self.id) {
            generate_object(obj, type_helper)
        } else {
            unreachable!()
        }
    }
}

pub fn generate_object(obj: &Object, type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
    let cls_name = &class_name(obj.name());
    quote! {
        class $cls_name {
            final Api _api;
            final Pointer<Void> _ptr;

            $(cls_name)._(this._api, this._ptr);

            factory $(cls_name).lift(Api api, Pointer<Void> ptr) {
                return $(cls_name)._(api, ptr);
            }

            Pointer<Void> uniffiClonePointer() {
                return rustCall(_api, (status) => _api.$(obj.ffi_object_clone().name())(_ptr, status));
            }

            void drop() {
                rustCall(_api, (status) => _api.$(obj.ffi_object_free().name())(_ptr, status));
            }

            $(for mt in &obj.methods() => $(generate_method(mt, type_helper)))
        }

        class $(obj.as_codetype().ffi_converter_name()) {
            static $cls_name lift(Api api, Pointer<Void> ptr) {
                return $cls_name.lift(api, ptr);
            }

            static Pointer<Void> lower(Api api, $cls_name value) {
                return value.uniffiClonePointer();
            }

            static void destroy(Api api, Pointer<Void> ptr) {
                rustCall(api, (status) => api.$(obj.ffi_object_free().name())(ptr, status));
            }
        }
    }
}

pub fn generate_method(func: &Method, type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
    let args = quote!($(for arg in &func.arguments() => $(&arg.as_renderable().render_type(&arg.as_type(), type_helper)) $(var_name(arg.name())),));

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
                  () => _api.$(func.ffi_func().name())(
                    uniffiClonePointer(),
                    $(for arg in &func.arguments() => $(DartCodeOracle::type_lower_fn(&arg.as_type(), quote!($(var_name(arg.name()))))),)
                  ),
                  _api.$(func.ffi_rust_future_poll(type_helper.get_ci()).name()),
                  _api.$(func.ffi_rust_future_complete(type_helper.get_ci()).name()),
                  _api.$(func.ffi_rust_future_free(type_helper.get_ci()).name()),
                  $lifter,
                );
            }
        )
    } else {
        quote!(
            $ret $(DartCodeOracle::fn_name(func.name()))($args) {
                return rustCall(_api, (status) => $lifter(_api.$(func.ffi_func().name())(
                    uniffiClonePointer(),
                    $(for arg in &func.arguments() => $(DartCodeOracle::type_lower_fn(&arg.as_type(), quote!($(var_name(arg.name()))))),) status
                )));
            }
        )
    }
}

