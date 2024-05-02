use genco::prelude::*;
use uniffi_bindgen::backend::{CodeType, Literal};
use uniffi_bindgen::interface::{AsType, Record};

use super::oracle::DartCodeOracle;
use super::render::{Renderable, TypeHelperRenderer};
use super::types::generate_type;
use super::utils::{class_name, var_name};

#[derive(Debug)]
pub struct RecordCodeType {
    id: String,
    module_path: String,
}

impl RecordCodeType {
    pub fn new(id: String, module_path: String) -> Self {
        Self { id, module_path }
    }
}

impl CodeType for RecordCodeType {
    fn type_label(&self) -> String {
        DartCodeOracle::class_name(&self.id)
    }

    fn canonical_name(&self) -> String {
        self.id.to_string()
    }

    fn literal(&self, _literal: &Literal) -> String {
        todo!("literal not implemented");
    }

    fn ffi_converter_name(&self) -> String {
        self.canonical_name().to_string() // Objects will use factory methods
    }
}

impl Renderable for RecordCodeType {
    fn render_type_helper(&self, type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
        if type_helper.check(&self.id) {
            quote!()
        } else if let Some(record_) = type_helper.get_record(&self.id) {
            generate_record(record_)
        } else {
            todo!("render_type_helper not implemented");
        }
    }
}

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
