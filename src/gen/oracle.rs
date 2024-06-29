use genco::lang::dart;
use genco::quote;
use heck::{ToLowerCamelCase, ToUpperCamelCase};

use uniffi_bindgen::backend::CodeType;
use uniffi_bindgen::interface::{AsType, Callable, ExternalKind, FfiType, Type};
use uniffi_bindgen::ComponentInterface;

use crate::gen::primitives;

use super::render::{AsRenderable, Renderable};
use super::{compounds, enums, objects, records};

pub struct DartCodeOracle;

impl DartCodeOracle {
    // ... (existing methods remain the same)

    pub fn convert_from_rust_buffer(ty: &Type, inner: dart::Tokens) -> dart::Tokens {
        match ty {
            Type::Object { .. } => inner,
            Type::String | Type::Optional { .. } => quote!($(inner).asUint8List()),
            _ => inner,
        }
    }

    pub fn convert_to_rust_buffer(ty: &Type, inner: dart::Tokens) -> dart::Tokens {
        match ty {
            Type::Object { .. } => inner,
            Type::String | Type::Optional { .. } | Type::Enum { .. } | Type::Sequence { .. } => {
                quote!(toRustBuffer(api, $inner))
            }
            _ => inner,
        }
    }

    pub fn type_lift_fn(ty: &Type, inner: dart::Tokens) -> dart::Tokens {
        match ty {
            Type::Int8
            | Type::UInt8
            | Type::Int16
            | Type::UInt16
            | Type::Int32
            | Type::Int64
            | Type::UInt32
            | Type::UInt64
            | Type::Float32
            | Type::Float64 => inner,
            Type::Boolean
            | Type::Duration
            | Type::String
            | Type::Object { .. }
            | Type::Enum { .. }
            | Type::Record { .. }
            | Type::Optional { .. } => quote!($(ty.as_codetype().ffi_converter_name()).lift(api, $inner)),
            _ => todo!("lift Type::{:?}", ty),
        }
    }

    pub fn type_lower_fn(ty: &Type, inner: dart::Tokens) -> dart::Tokens {
        match ty {
            Type::UInt32
            | Type::Int8
            | Type::UInt8
            | Type::Int16
            | Type::UInt16
            | Type::Int32
            | Type::Int64
            | Type::UInt64
            | Type::Float32
            | Type::Float64 => inner,
            Type::Boolean
            | Type::Duration
            | Type::String
            | Type::Object { .. }
            | Type::Enum { .. }
            | Type::Optional { .. }
            | Type::Record { .. }
            | Type::Sequence { .. } => quote!($(ty.as_codetype().ffi_converter_name()).lower(api, $inner)),
            _ => todo!("lower Type::{:?}", ty),
        }
    }

    // ... (rest of the file remains the same)
}

