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
        self.canonical_name().to_string() // Objects will use factory methods
    }
}

impl Renderable for ObjectCodeType {
    // Semantically, it may make sense to render object here, but we don't have enough information. So we render it with help from type_helper
    fn render(&self) -> dart::Tokens {
        quote!()
    }

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

// Let's refactor this later
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

            Pointer<Void> uniffiClonePointer(Api, api) {
                return rustCall(api, (status) => $(DartCodeOracle::find_lib_instance()).$(obj.ffi_object_clone().name())(_ptr, status));
            }

            void drop(Api api) {
                rustCall(api, (status) => $(DartCodeOracle::find_lib_instance())..$(obj.ffi_object_free().name())(_ptr, status));
            }

            $(for mt in &obj.methods() => $(generate_method(mt, type_helper)))
        }
    }
}

#[allow(unused_variables)]
pub fn generate_method(fun: &Method, type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
    let api = "_api";
    let ffi = fun.ffi_func();
    let fn_name = fn_name(fun.name());
    let args = quote!($(for arg in &fun.arguments() => $(&arg.as_renderable().render_type(&arg.as_type(), type_helper)) $(var_name(arg.name())),));
    let ff_name = ffi.name();
    let inner = quote! {
    rustCall(api, (res) =>
        _$(&fn_name)(
            uniffiClonePointer(),
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
        (quote!(void), quote!($inner;))
    };

    quote! {
        $ret $fn_name ($args) {
            final api = _api;
            $body
        }
    }
}
