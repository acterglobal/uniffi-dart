use std::{
    cell::RefCell,
    collections::{BTreeSet, HashMap},
};

use genco::prelude::*;
use uniffi_bindgen::{
    interface::{FfiType, Type},
    ComponentInterface,
};

use super::{
    render::{AsRenderable, Renderer, TypeHelperRenderer},
    Config,
};

type FunctionDefinition = dart::Tokens;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ImportRequirement {
    Import { name: String },
    ImportAs { name: String, as_name: String },
}

pub struct TypeHelpersRenderer<'a> {
    config: &'a Config,
    ci: &'a ComponentInterface,
    include_once_names: RefCell<HashMap<String, Type>>,
    imports: RefCell<BTreeSet<ImportRequirement>>,
}

impl<'a> TypeHelpersRenderer<'a> {
    pub fn new(config: &'a Config, ci: &'a ComponentInterface) -> Self {
        Self {
            config,
            ci,
            include_once_names: RefCell::new(HashMap::new()),
            imports: RefCell::new(BTreeSet::new()),
        }
    }

    pub fn external_type_package_name(&self, crate_name: &str) -> String {
        self.config.external_packages.get(crate_name).cloned()
            .unwrap_or_else(|| crate_name.to_string())
    }

    // ... (other methods remain the same)
}

impl TypeHelperRenderer for TypeHelpersRenderer<'_> {
    // ... (existing methods remain the same)

    fn ffi_type_label(&self, ffi_type: &FfiType) -> dart::Tokens {
        match ffi_type {
            FfiType::Int8 => quote!(int),
            FfiType::UInt8 => quote!(int),
            FfiType::Int16 => quote!(int),
            FfiType::UInt16 => quote!(int),
            FfiType::Int32 => quote!(int),
            FfiType::UInt32 => quote!(int),
            FfiType::Int64 => quote!(int),
            FfiType::UInt64 => quote!(int),
            FfiType::Float32 => quote!(double),
            FfiType::Float64 => quote!(double),
            FfiType::RustBuffer(_) => quote!(RustBuffer),
            FfiType::RustArcPtr(_) => quote!(Pointer<Void>),
            FfiType::ForeignBytes => quote!(ForeignBytes),
            _ => todo!("FFI type not implemented: {:?}", ffi_type),
        }
    }

    fn ffi_native_type_label(&self, ffi_type: &FfiType) -> dart::Tokens {
        match ffi_type {
            FfiType::Int8 => quote!(Int8),
            FfiType::UInt8 => quote!(Uint8),
            FfiType::Int16 => quote!(Int16),
            FfiType::UInt16 => quote!(Uint16),
            FfiType::Int32 => quote!(Int32),
            FfiType::UInt32 => quote!(Uint32),
            FfiType::Int64 => quote!(Int64),
            FfiType::UInt64 => quote!(Uint64),
            FfiType::Float32 => quote!(Float),
            FfiType::Float64 => quote!(Double),
            FfiType::RustBuffer(_) => quote!(RustBuffer),
            FfiType::RustArcPtr(_) => quote!(Pointer<Void>),
            FfiType::ForeignBytes => quote!(ForeignBytes),
            _ => todo!("Native FFI type not implemented: {:?}", ffi_type),
        }
    }
}

// ... (rest of the file remains the same)

