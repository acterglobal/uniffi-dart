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
    pub fn find(type_: &Type) -> Box<dyn CodeType> {
        type_.clone().as_type().as_codetype()
    }

    pub fn find_renderable(type_: &Type) -> Box<dyn Renderable> {
        type_.clone().as_type().as_renderable()
    }

    pub fn find_as_error(type_: &Type) -> Box<dyn CodeType> {
        panic!("unsupported type for error: {type_:?}")
    }

    /// Sanitize a Dart identifier, appending an underscore if it's a reserved keyword.
    pub fn sanitize_identifier(id: &str) -> String {
        if Self::is_reserved_identifier(id) {
            format!("{}_", id)
        } else {
            id.to_string()
        }
    }

    /// Check if the given identifier is a reserved keyword in Dart.
    pub fn is_reserved_identifier(id: &str) -> bool {
        RESERVED_IDENTIFIERS.contains(&id)
    }

    /// Get the idiomatic Dart rendering of a class name (for enums, records, errors, etc).
    pub fn class_name(nm: &str) -> String {
        Self::sanitize_identifier(&nm.to_upper_camel_case())
    }

    /// Get the idiomatic Dart rendering of a function name.
    pub fn fn_name(nm: &str) -> String {
        Self::sanitize_identifier(&nm.to_lower_camel_case())
    }

    /// Get the idiomatic Dart rendering of a variable name.
    pub fn var_name(nm: &str) -> String {
        Self::sanitize_identifier(&nm.to_lower_camel_case())
    }

    /// Get the idiomatic Dart rendering of an individual enum variant.
    pub fn enum_variant_name(nm: &str) -> String {
        Self::sanitize_identifier(&nm.to_lower_camel_case())
    }

    /// Get the idiomatic Dart rendering of an FFI callback function name
    fn ffi_callback_name(nm: &str) -> String {
        format!("Pointer<NativeFunction<Uniffi{}>>", nm.to_upper_camel_case())
    }

    /// Get the idiomatic Dart rendering of an exception name
    pub fn error_name(nm: &str) -> String {
        let name = Self::class_name(nm);
        match name.strip_suffix("Error") {
            None => name,
            Some(stripped) => format!("{stripped}Exception"),
        }
    }

    pub fn find_lib_instance() -> dart::Tokens {
        quote!(_UniffiLib.instance)
    }

    pub fn async_poll(
        callable: impl Callable,
        ci: &ComponentInterface,
    ) -> dart::Tokens {
        let ffi_func = callable.ffi_rust_future_poll(ci);
        quote!($(Self::find_lib_instance()).$ffi_func)
    }

    pub fn async_complete(
        callable: impl Callable,
        ci: &ComponentInterface,
    ) -> dart::Tokens {
        let ffi_func = callable.ffi_rust_future_complete(ci);
        let call = quote!($(Self::find_lib_instance()).$ffi_func);
        let call = match callable.return_type() {
            Some(Type::External {
                kind: ExternalKind::DataClass,
                name,
                ..
            }) => {
                todo!("Need to convert the RustBuffer from our package to the RustBuffer of the external package")
            }
            _ => call,
        };
        call
    }

    pub fn async_free(
        callable: impl Callable,
        ci: &ComponentInterface,
    ) -> dart::Tokens {
        let ffi_func = callable.ffi_rust_future_free(ci);
        quote!($(Self::find_lib_instance()).$ffi_func)
    }

    pub fn ffi_dart_type_label(ffi_type: Option<&FfiType>) -> dart::Tokens {
        let Some(ret_type) = ffi_type else {
            return quote!(void);
        };
        match ret_type {
            FfiType::UInt8 |
            FfiType::UInt16 |
            FfiType::UInt32 |
            FfiType::UInt64 |
            FfiType::Int8 |
            FfiType::Int16 |
            FfiType::Int32 |
            FfiType::Handle |
            FfiType::Int64 => quote!(int),
            FfiType::Float32 | FfiType::Float64 => quote!(double),
            FfiType::RustBuffer(ref inner) => match inner {
                Some(i) => quote!($i),
                _ => quote!(RustBuffer),
            },
            FfiType::ForeignBytes => quote!(ForeignBytes),
            FfiType::RustArcPtr(_) => quote!(Pointer<Void>),
            FfiType::Callback (name) => quote!($(Self::ffi_callback_name(name))),
            _ => todo!("FfiType::{:?}", ret_type),
        }
    }

    pub fn ffi_native_type_label(ffi_ret_type: Option<&FfiType>) -> dart::Tokens {
        let Some(ret_type) = ffi_ret_type else {
            return quote!(Void)
        };
        match ret_type {
            FfiType::UInt8 => quote!(Uint8),
            FfiType::UInt16 => quote!(Uint16),
            FfiType::UInt32 => quote!(Uint32),
            FfiType::UInt64 => quote!(Uint64),
            FfiType::Int8 => quote!(Int8),
            FfiType::Int16 => quote!(Int16),
            FfiType::Int32 => quote!(Int32),
            FfiType::Int64 => quote!(Int64),
            FfiType::Float32 => quote!(Float),
            FfiType::Float64 => quote!(Double),
            FfiType::Handle => quote!(Uint64),
            FfiType::RustBuffer(ref inner) => match inner {
                Some(i) => quote!($i),
                _ => quote!(RustBuffer),
            },
            FfiType::ForeignBytes => quote!(ForeignBytes),
            FfiType::RustArcPtr(_) => quote!(Pointer<Void>),
            FfiType::Callback (name) => quote!($(Self::ffi_callback_name(name))),
            _ => todo!("FfiType::{:?}", ret_type),
        }
    }

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
                quote!(toRustBuffer($inner))
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
            | Type::Optional { .. } => quote!($(ty.as_codetype().ffi_converter_name()).lift($inner)),
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
            | Type::Sequence { .. } => quote!($(ty.as_codetype().ffi_converter_name()).lower($inner)),
            _ => todo!("lower Type::{:?}", ty),
        }
    }
}

pub static RESERVED_IDENTIFIERS: [&str; 63] = [
    "abstract", "as", "assert", "async", "await", "break", "case", "catch", "class", "const",
    "continue", "covariant", "default", "deferred", "do", "dynamic", "else", "enum", "export",
    "extends", "extension", "external", "factory", "false", "final", "finally", "for", "Function",
    "get", "hide", "if", "implements", "import", "in", "interface", "is", "late", "library",
    "mixin", "new", "null", "on", "operator", "part", "required", "rethrow", "return", "set",
    "show", "static", "super", "switch", "sync", "this", "throw", "true", "try", "typedef",
    "var", "void", "while", "with", "yield",
];

pub trait AsCodeType {
    fn as_codetype(&self) -> Box<dyn CodeType>;
}

impl<T: AsType> AsCodeType for T {
    fn as_codetype(&self) -> Box<dyn CodeType> {
        match self.as_type() {
            Type::UInt8 => Box::new(primitives::UInt8CodeType),
            Type::Int8 => Box::new(primitives::Int8CodeType),
            Type::UInt16 => Box::new(primitives::UInt16CodeType),
            Type::Int16 => Box::new(primitives::Int16CodeType),
            Type::UInt32 => Box::new(primitives::UInt32CodeType),
            Type::Int32 => Box::new(primitives::Int32CodeType),
            Type::UInt64 => Box::new(primitives::UInt64CodeType),
            Type::Int64 => Box::new(primitives::Int64CodeType),
            Type::Float32 => Box::new(primitives::Float32CodeType),
            Type::Float64 => Box::new(primitives::Float64CodeType),
            Type::Boolean => Box::new(primitives::BooleanCodeType),
            Type::String => Box::new(primitives::StringCodeType),
            Type::Duration => Box::new(primitives::DurationCodeType),
            Type::Object { name, .. } => Box::new(objects::ObjectCodeType::new(name)),
            Type::Optional { inner_type } => Box::new(compounds::OptionalCodeType::new(
                self.as_type(),
                *inner_type,
            )),
            Type::Sequence { inner_type } => Box::new(compounds::SequenceCodeType::new(
                self.as_type(),
                *inner_type,
            )),
            Type::Enum { name, .. } => Box::new(enums::EnumCodeType::new(name)),
            Type::Record { name, module_path } => {
                Box::new(records::RecordCodeType::new(name, module_path))
            }
            _ => todo!("As Type for Type::{:?}", self.as_type()),
        }
    }
}

