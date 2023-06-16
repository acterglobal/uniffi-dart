use heck::{ToLowerCamelCase, ToUpperCamelCase};
use uniffi_bindgen::{
    backend::{CodeType, TypeIdentifier},
    interface::{FfiType, Type},
};

use super::primitives;

pub fn sanitize_identifier(id: &str) -> String {
    if RESERVED_IDENTIFIERS.contains(&id) {
        format!("{}_", id)
    } else {
        id.to_string()
    }
}

pub fn create_code_type(type_: TypeIdentifier) -> Box<dyn CodeType> {
    match type_ {
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

        // Type::Timestamp => Box::new(miscellany::TimestampCodeType),
        // Type::Duration => Box::new(miscellany::DurationCodeType),

        // Type::Enum(id) => Box::new(enum_::EnumCodeType::new(id)),
        // Type::Object(id) => Box::new(object::ObjectCodeType::new(id)),
        // Type::Record(id) => Box::new(record::RecordCodeType::new(id)),
        // Type::Error(id) => Box::new(error::ErrorCodeType::new(id)),
        // Type::CallbackInterface(id) => {
        //     Box::new(callback_interface::CallbackInterfaceCodeType::new(id))
        // }

        // Type::Optional(inner) => Box::new(compounds::OptionalCodeType::new(*inner)),
        // Type::Sequence(inner) => Box::new(compounds::SequenceCodeType::new(*inner)),
        // Type::Map(key, value) => Box::new(compounds::MapCodeType::new(*key, *value)),
        // Type::External { name, .. } => Box::new(external::ExternalCodeType::new(name)),
        // Type::Custom { name, .. } => Box::new(custom::CustomCodeType::new(name)),
        Type::Unresolved { name } => {
            unreachable!("Type `{name}` must be resolved before calling create_code_type")
        }
        _ => todo!("Type not yet implemented"),
    }
}

pub fn find(type_: &TypeIdentifier) -> Box<dyn CodeType> {
    create_code_type(type_.clone())
}

/// Get the idiomatic Dart rendering of a class name (for enums, records, errors, etc).
pub fn class_name(nm: &str) -> String {
    sanitize_identifier(&nm.to_upper_camel_case())
}

/// Get the idiomatic Dart rendering of a function name.
pub fn fn_name(nm: &str) -> String {
    sanitize_identifier(&nm.to_lower_camel_case())
}

/// Get the idiomatic Dart rendering of a variable name.
pub fn var_name(nm: &str) -> String {
    sanitize_identifier(&nm.to_lower_camel_case())
}

/// Get the idiomatic Dart rendering of an individual enum variant.
pub fn enum_variant_name(nm: &str) -> String {
    sanitize_identifier(&nm.to_lower_camel_case())
}

/// Get the idiomatic Dart rendering of an exception name.
pub fn error_name(nm: &str) -> String {
    class_name(nm)
}

pub fn ffi_type_label(ffi_type: &FfiType) -> String {
    match ffi_type {
        FfiType::Int8 => "int8_t".into(),
        FfiType::UInt8 => "uint8_t".into(),
        FfiType::Int16 => "int16_t".into(),
        FfiType::UInt16 => "uint16_t".into(),
        FfiType::Int32 => "int32_t".into(),
        FfiType::UInt32 => "uint32_t".into(),
        FfiType::Int64 => "int64_t".into(),
        FfiType::UInt64 => "uint64_t".into(),
        FfiType::Float32 => "float".into(),
        FfiType::Float64 => "double".into(),
        FfiType::RustArcPtr(_) => "void*_Nonnull".into(),
        FfiType::RustBuffer(_) => "RustBuffer".into(),
        FfiType::ForeignBytes => "ForeignBytes".into(),
        FfiType::ForeignCallback => "ForeignCallback _Nonnull".to_string(),
    }
}

// https://dart.dev/guides/language/language-tour#keywords
pub static RESERVED_IDENTIFIERS: [&str; 63] = [
    "abstract",
    "as",
    "assert",
    "async",
    "await",
    "break",
    "case",
    "catch",
    "class",
    "const",
    "continue",
    "covariant",
    "default",
    "deferred",
    "do",
    "dynamic",
    "else",
    "enum",
    "export",
    "extends",
    "extension",
    "external",
    "factory",
    "false",
    "final",
    "finally",
    "for",
    "Function",
    "get",
    "hide",
    "if",
    "implements",
    "import",
    "in",
    "interface",
    "is",
    "late",
    "library",
    "mixin",
    "new",
    "null",
    "on",
    "operator",
    "part",
    "required",
    "rethrow",
    "return",
    "set",
    "show",
    "static",
    "super",
    "switch",
    "sync",
    "this",
    "throw",
    "true",
    "try",
    "typedef",
    "var",
    "void",
    "while",
    "with",
    "yield",
];
