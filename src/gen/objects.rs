use genco::prelude::*;
use uniffi_bindgen::backend::Literal;
use uniffi_bindgen::interface::{AsType, Method, Object};

use crate::gen::oracle::{AsCodeType, DartCodeOracle};
use crate::gen::render::AsRenderable;
use crate::gen::render::{Renderable, TypeHelperRenderer};
use crate::gen::CodeType;

use super::stream::generate_stream;

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
        self.canonical_name().to_string() // Objects will use factory methods
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
    type_helper.include_once_check(obj.name(), &obj.as_type());

    let cls_name = &DartCodeOracle::class_name(obj.name());
    let finalizer_cls_name = &format!("{}Finalizer", cls_name);
    let lib_instance = &DartCodeOracle::find_lib_instance();
    let ffi_object_free_name = obj.ffi_object_free().name();
    let ffi_object_clone_name = obj.ffi_object_clone().name();

    // Stream workaround, make it more elegant later

    let stream_glue = if obj.name().contains("StreamExt") {
        generate_stream(obj, type_helper)
    } else {
        quote!()
    };

    quote! {
        final _$finalizer_cls_name = Finalizer<Pointer<Void>>((ptr) {
          rustCall((status) => $lib_instance.$ffi_object_free_name(ptr, status));
        });

        class $cls_name {
            final Pointer<Void> _ptr;

            $(cls_name)._(this._ptr) {
                _$finalizer_cls_name.attach(this, _ptr, detach: this);
            }

            factory $(cls_name).lift(Pointer<Void> ptr) {
                return $(cls_name)._(ptr);
            }

            Pointer<Void> uniffiClonePointer() {
                return rustCall((status) => $lib_instance.$ffi_object_clone_name(_ptr, status));
            }

            void dispose() {
                _$finalizer_cls_name.detach(this);
                rustCall((status) => $lib_instance.$ffi_object_free_name(_ptr, status));
            }

            $(for mt in &obj.methods() => $(generate_method(mt, type_helper)))
        }

        $(stream_glue)
    }
}

#[allow(unused_variables)]
pub fn generate_method(func: &Method, type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
    // if func.takes_self_by_arc() {} // TODO: Do something about this condition
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
                    uniffiClonePointer(),
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
                    uniffiClonePointer(),
                    $(for arg in &func.arguments() => $(DartCodeOracle::type_lower_fn(&arg.as_type(), quote!($(DartCodeOracle::var_name(arg.name()))))),) status
                )));
            }
        )
    }
}
