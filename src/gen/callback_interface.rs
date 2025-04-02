use genco::prelude::*;
use uniffi_bindgen::backend::CodeType;
use crate::gen::render::{Renderable, TypeHelperRenderer};

#[derive(Debug)]
pub struct CallbackInterfaceCodeType {
    name: String,
}

impl CallbackInterfaceCodeType {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl CodeType for CallbackInterfaceCodeType {
    fn type_label(&self) -> String {
        super::DartCodeOracle::class_name(&self.name)
    }

    fn canonical_name(&self) -> String {
        format!("CallbackInterface{}", self.type_label())
    }

    fn initialization_fn(&self) -> Option<String> {
        Some(format!("_uniffiInitializeCallbackInterface{}", self.name))
    }
}

impl Renderable for CallbackInterfaceCodeType {
    fn render_type_helper(&self, _type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
        quote!("Plese start here")
    }
}
