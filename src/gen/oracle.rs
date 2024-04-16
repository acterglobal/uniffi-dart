use genco::lang::dart;
use genco::quote;
use heck::{ToLowerCamelCase, ToUpperCamelCase};

use uniffi_bindgen::backend::CodeType;
use uniffi_bindgen::interface::{AsType, FfiType, Literal, Type};

use crate::gen::primitives;

use super::render::{AsRenderable, Renderable};
use super::{compounds, enums, objects};

pub struct DartCodeOracle;

impl DartCodeOracle {
    pub fn find(type_: &Type) -> Box<dyn CodeType> {
        type_.clone().as_type().as_codetype()
    }

    pub fn find_renderable(type_: &Type) -> Box<dyn Renderable> {
        type_.clone().as_type().as_renderable()
    }

    pub fn find_as_error(type_: &Type) -> Box<dyn CodeType> {
        match type_ {
            //Type::Enum(id) => Box::new(error::ErrorCodeType::new(id.clone())),
            _ => panic!("unsupported type for error: {type_:?}"),
        }
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

    /// Get the idiomatic Dart rendering of an exception name
    // TODO: Refactor to be more idomatic to the way dart handles errors
    pub fn error_name(nm: &str) -> String {
        // errors are a class in Dart.
        let name = Self::class_name(nm);
        match name.strip_suffix("Error") {
            None => name,
            Some(stripped) => format!("{stripped}Exception"),
        }
    }

    // TODO: Replace instances of `generate_ffi_dart_type` with ffi_type_label
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
            FfiType::Int64 => quote!(int),
            FfiType::Float32 | FfiType::Float64 => quote!(double),
            FfiType::RustBuffer(ref inner) => match inner {
                Some(i) => quote!($i),
                _ => quote!(RustBuffer),
            },
            FfiType::ForeignExecutorHandle |
            FfiType::FutureCallbackData |
            FfiType::RustArcPtr(_) => quote!(Pointer<Void>),
            FfiType::FutureCallback { return_type   } => {
                quote!(UniFfiFutureCallback$(Self::ffi_native_type_label(Some(return_type))))
            }
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
            FfiType::RustBuffer(ref inner) => match inner {
                Some(i) => quote!($i),
                _ => quote!(RustBuffer),
            },
            FfiType::ForeignExecutorHandle |
            FfiType::FutureCallbackData |
            FfiType::RustArcPtr(_) => quote!(Pointer<Void>),
            FfiType::FutureCallback { return_type   } => {
                quote!(UniFfiFutureCallback$(Self::ffi_native_type_label(Some(return_type))))
            }
            _ => todo!("FfiType::{:?}", ret_type),
        }
    }

    // This function is equivalent to type_lable in code type
    // pub fn generate_type(ty: &Type) -> dart::Tokens {
    //     match ty {
    //         Type::UInt8
    //         | Type::UInt32
    //         | Type::Int8
    //         | Type::Int16
    //         | Type::Int64
    //         | Type::UInt16
    //         | Type::Int32
    //         | Type::UInt64 => quote!(int),
    //         Type::Float32 | Type::Float64 => quote!(double),
    //         Type::String => quote!(String),
    //         Type::Object{name, ..} => quote!($name),
    //         Type::Boolean => quote!(bool),
    //         Type::Optional( inner_type) => quote!($(generate_type(inner_type))?),
    //         Type::Sequence ( inner_type ) => quote!(List<$(generate_type(inner_type))>),
    //         Type::Enum ( name,..  ) => quote!($name),
    //         // Type::Record { name,..  } => quote!($name),
    //         _ => todo!("Type::{:?}", ty)
    //         // AbiType::Num(ty) => Self::generate_wrapped_num_type(*ty),
    //         // AbiType::Isize | AbiType::Usize => quote!(int),
    //         // AbiType::Bool => quote!(bool),
    //         // AbiType::RefStr | AbiType::String => quote!(String),
    //         // AbiType::RefSlice(ty) | AbiType::Vec(ty) => {
    //         //     quote!(List<#(Self::generate_wrapped_num_type(*ty))>)
    //         // }
    //         // AbiType::Option(ty) => quote!(#(Self::generate_type(ty))?),
    //         // AbiType::Result(ty) => Self::generate_type(ty),
    //         // AbiType::Tuple(tuple) => match tuple.len() {
    //         //     0 => quote!(void),
    //         //     1 => Self::generate_type(&tuple[0]),
    //         //     _ => quote!(List<dynamic>),
    //         // },
    //         // AbiType::RefObject(ty) | AbiType::Object(ty) => quote!(#ty),
    //         // AbiType::RefIter(ty) | AbiType::Iter(ty) => quote!(Iter<#(Self::generate_type(ty))>),
    //         // AbiType::RefFuture(ty) | AbiType::Future(ty) => {
    //         //     quote!(Future<#(Self::generate_type(ty))>)
    //         // }
    //         // AbiType::RefStream(ty) | AbiType::Stream(ty) => {
    //         //     quote!(Stream<#(Self::generate_type(ty))>)
    //         // }
    //         // AbiType::Buffer(ty) => quote!(#(ffi_buffer_name_for(*ty))),
    //         // AbiType::List(ty) => quote!(#(format!("FfiList{}", ty))),
    //         // AbiType::RefEnum(ty) => quote!(#(ty)),
    //     }
    // }
    // TODO: implement error_ffi_converter, future_callback handler, future continuation type, allocation size handler

    pub fn convert_from_rust_buffer(ty: &Type, inner: dart::Tokens) -> dart::Tokens {
        match ty {
            Type::Object { .. } => inner,
            Type::String | Type::Optional { .. } => quote!($(inner).toIntList()),
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
            | Type::String
            | Type::Object { .. }
            | Type::Enum { .. }
            | Type::Optional { .. } => quote!($(ty.as_codetype().lift())(api, $inner)),
            _ => todo!("lift Type::{:?}", ty),
        }
    }

    // fn type_lift_optional_inner_type(inner_type: &Box<Type>, inner: dart::Tokens) -> dart::Tokens {
    //     match **inner_type {
    //         Type::Int8 | Type::UInt8 => quote!(liftOptional(api, $inner, (api, v) => liftInt8OrUint8(v))),
    //         Type::Int16 | Type::UInt16 => quote!(liftOptional(api, $inner, (api, v) => liftInt16OrUint16(v))),
    //         Type::Int32 | Type::UInt32 => quote!(liftOptional(api, $inner, (api, v) => liftInt32OrUint32(v))),
    //         Type::Int64 | Type::UInt64 => quote!(liftOptional(api, $inner, (api, v) => liftInt64OrUint64(v))),
    //         Type::Float32 => quote!(liftOptional(api, $inner, (api, v) => liftFloat32(v))),
    //         Type::Float64 => quote!(liftOptional(api, $inner, (api, v) => liftFloat64(v))),
    //         Type::String => quote!(liftOptional(api, $inner, (api, v) => $(Self::type_lift_fn(inner_type, quote!(v.sublist(5))))) ),
    //         _ => todo!("lift Option inner type: Type::{:?}", inner_type)
    //     }
    // }

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
            | Type::String
            | Type::Object { .. }
            | Type::Enum { .. }
            | Type::Optional { .. }
            | Type::Sequence { .. } => quote!($(ty.as_codetype().lower())(api, $inner)),
            //      => quote!(lowerSequence(api, value, lowerUint8, 1)), // TODO: Write try lower primitives, then check what a sequence actually looks like and replicate it
            _ => todo!("lower Type::{:?}", ty),
        }
    }
}

// https://dart.dev/guides/language/language-tour#keywords
pub static RESERVED_IDENTIFIERS: [&str; 63] = [
    // This list may need to be updated as the Dart language evolves.
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
            _ => todo!("As Type for Type::{:?}", self.as_type()), // Type::Bytes => Box::new(primitives::BytesCodeType),

                                                                  // Type::Timestamp => Box::new(miscellany::TimestampCodeType),
                                                                  // Type::Duration => Box::new(miscellany::DurationCodeType),

                                                                  // ,
                                                                  // Type::Object { name, .. } => Box::new(object::ObjectCodeType::new(name)),
                                                                  // Type::Record(id) => Box::new(record::RecordCodeType::new(id)),
                                                                  // Type::CallbackInterface(id) => {
                                                                  //     Box::new(callback_interface::CallbackInterfaceCodeType::new(id))
                                                                  // }
                                                                  // Type::ForeignExecutor => Box::new(executor::ForeignExecutorCodeType),
                                                                  // Type::Optional(inner) => Box::new(compounds::OptionalCodeType::new(*inner)),
                                                                  // ,
                                                                  // Type::Map(key, value) => Box::new(compounds::MapCodeType::new(*key, *value)),
                                                                  // Type::External { name, .. } => Box::new(external::ExternalCodeType::new(name)),
                                                                  // Type::Custom { name, .. } => Box::new(custom::CustomCodeType::new(name)),
        }
    }
}
