use genco::lang::dart;
use genco::quote;
use heck::{ToLowerCamelCase, ToUpperCamelCase};
use uniffi_bindgen::backend::CodeType;
use uniffi_bindgen::interface::{AsType, Callable, ExternalKind, FfiType, Type};
use uniffi_bindgen::ComponentInterface;

use crate::gen::primitives;

// use super::render::{AsRenderable, Renderable};
use super::{callback_interface, compounds, enums, objects, records};

pub struct DartCodeOracle;

impl DartCodeOracle {
    pub fn find(type_: &Type) -> Box<dyn CodeType> {
        type_.clone().as_type().as_codetype()
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
        let name = Self::sanitize_identifier(&nm.to_upper_camel_case());
        match name.strip_suffix("Error") {
            None => name,
            Some(stripped) => format!("{stripped}Exception"),
        }
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
        format!(
            "Pointer<NativeFunction<{}>>",
            Self::callback_name(&nm.to_upper_camel_case())
        )
    }

    /// Helper method to generate the callback name based on `Type`.
    fn callback_name(name: &str) -> String {
        format!("Uniffi{}", name.to_upper_camel_case())
    }

    /// Helper method to generate the struct name based on `Type`.
    fn struct_name(name: &str) -> String {
        format!("Uniffi{}", name.to_upper_camel_case())
    }


    /// Helper method to generate external Dart type labels.
    fn external_type_label(name: &str) -> String {
        format!("External{}", name.to_upper_camel_case())
    }

    /// Helper method to generate external native type labels.
    fn external_native_type_label(name: &str) -> String {
        format!("Pointer<{}>", Self::external_type_label(name))
    }
    /// Get the idiomatic Dart rendering of an exception name
    // pub fn error_name(nm: &str) -> String {
    //     let name = Self::class_name(nm);
    //     match name.strip_suffix("Error") {
    //         None => name,
    //         Some(stripped) => format!("{stripped}Exception"),
    //     }
    // }

    pub fn find_lib_instance() -> dart::Tokens {
        quote!(_UniffiLib.instance)
    }

    // TODO: Replace instances of `generate_ffi_dart_type` with ffi_type_label
    pub fn ffi_dart_type_label(ffi_type: Option<&FfiType>) -> dart::Tokens {
        let Some(ret_type) = ffi_type else {
            return quote!(void);
        };
        match ret_type {
            FfiType::UInt8
            | FfiType::UInt16
            | FfiType::UInt32
            | FfiType::UInt64
            | FfiType::Int8
            | FfiType::Int16
            | FfiType::Int32
            | FfiType::Handle
            | FfiType::Int64 => quote!(int),
            FfiType::Float32 | FfiType::Float64 => quote!(double),
            FfiType::RustBuffer(ref inner) => match inner {
                Some(i) => quote!($i),
                _ => quote!(RustBuffer),
            },
            FfiType::ForeignBytes => quote!(ForeignBytes),
            FfiType::RustArcPtr(_) => quote!(Pointer<Void>),
            FfiType::Callback(name) => quote!($(Self::ffi_callback_name(name))),
            FfiType::Reference(inner) => quote!($(Self::ffi_type_label_by_reference(inner))),
            _ => todo!("FfiType::{:?}", ret_type),
        }
    }

    pub fn ffi_native_type_label(ffi_ret_type: Option<&FfiType>) -> dart::Tokens {
        let Some(ret_type) = ffi_ret_type else {
            return quote!(Void);
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
            FfiType::Callback(name) => quote!($(Self::ffi_callback_name(name))),
            FfiType::Reference(inner) => quote!($(Self::ffi_type_label_by_reference(inner))),
            _ => todo!("FfiType::{:?}", ret_type),
        }
    }

    fn ffi_type_label_by_reference(ffi_type: &FfiType) -> dart::Tokens {
        match ffi_type {
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
            FfiType::RustBuffer(_) => quote!(RustBuffer),
            FfiType::ForeignBytes => quote!(ForeignBytes),
            FfiType::RustArcPtr(_) => quote!(Pointer<Void>),
            FfiType::Callback(name) => quote!(Pointer<$(Self::ffi_callback_name(name))>),
            FfiType::Struct(name) => quote!(Pointer<$(Self::ffi_struct_name(name))>),
            _ => todo!("FfiType::{:?}", ffi_type),
        }
    }

    pub fn ffi_struct_name(name: &str) -> dart::Tokens {
       quote!($(format!("Uniffi{}", name.to_upper_camel_case())))
    }

    
    // pub fn convert_from_rust_buffer(ty: &Type, inner: dart::Tokens) -> dart::Tokens {
    //     match ty {
    //         Type::Object { .. } => inner,
    //         Type::String | Type::Optional { .. } => quote!($(inner).asUint8List()),
    //         _ => inner,
    //     }
    // }

    // pub fn convert_to_rust_buffer(ty: &Type, inner: dart::Tokens) -> dart::Tokens {
    //     match ty {
    //         Type::Object { .. } => inner,
    //         Type::String | Type::Optional { .. } | Type::Enum { .. } | Type::Sequence { .. } => {
    //             quote!(toRustBuffer($inner))
    //         }
    //         _ => inner,
    //     }
    // }

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
            | Type::Optional { .. } => {
                quote!($(ty.as_codetype().ffi_converter_name()).lift($inner))
            }
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
            | Type::Sequence { .. }
            | Type::CallbackInterface { .. } => {
                quote!($(ty.as_codetype().ffi_converter_name()).lower($inner))
            }
            _ => todo!("lower Type::{:?}", ty),
        }
    }

    pub fn async_poll(callable: impl Callable, ci: &ComponentInterface) -> dart::Tokens {
        let ffi_func = callable.ffi_rust_future_poll(ci);
        quote!($(Self::find_lib_instance()).$ffi_func)
    }

    pub fn async_complete(callable: impl Callable, ci: &ComponentInterface) -> dart::Tokens {
        let ffi_func = callable.ffi_rust_future_complete(ci);
        let call = quote!($(Self::find_lib_instance()).$ffi_func);
        match callable.return_type() {
            Some(Type::External {
                kind: ExternalKind::DataClass,
                name: _,
                ..
            }) => {
                todo!("Need to convert the RustBuffer from our package to the RustBuffer of the external package")
            }
            _ => call,
        }
    }

    pub fn async_free(callable: impl Callable, ci: &ComponentInterface) -> dart::Tokens {
        let ffi_func = callable.ffi_rust_future_free(ci);
        quote!($(Self::find_lib_instance()).$ffi_func)
    }

    /// Get the idiomatic Dart rendering of a class name based on `Type`.
    pub fn dart_type_label(type_: Option<&Type>) -> dart::Tokens {
        let Some(ret_type) = type_ else {
            return quote!(Void);
        };
        match ret_type {
            Type::UInt8
            | Type::UInt16
            | Type::UInt32
            | Type::UInt64
            | Type::Int8
            | Type::Int16
            | Type::Int32
            | Type::Int64 => quote!(int),
            Type::Float32 | Type::Float64 => quote!(double),
            Type::Boolean => quote!(int),
            Type::String => quote!(RustBuffer),
            Type::Bytes => quote!(RustBuffer),
            Type::Timestamp => quote!(RustBuffer),
            Type::Duration => quote!(RustBuffer),
            Type::Object { name, .. } => {
                let class_name = Self::class_name(name);
                quote!($class_name)
            }
            Type::Record { name, .. } => {
                let class_name = Self::class_name(name);
                quote!($class_name)
            }
            Type::Enum { name, .. } => {
                let enum_name = Self::class_name(name);
                quote!($enum_name)
            }
            Type::CallbackInterface { name, .. } => {
                let callback_name = Self::callback_name(name);
                quote!($callback_name)
            }
            Type::Optional{inner_type, ..} => {
                let inner_label = Self::dart_type_label(Some(inner_type.as_ref()));
                quote!($inner_label?)
            }
            Type::Sequence{inner_type, ..} => {
                let inner_label = Self::dart_type_label(Some(inner_type.as_ref()));
                quote!(List<$inner_label>)
            }
            Type::Map { key_type, value_type, .. } => {
                let key_label = Self::dart_type_label(Some(key_type.as_ref()));
                let value_label = Self::dart_type_label(Some(value_type.as_ref()));
                quote!(Map<$key_label, $value_label>)
            }
            Type::External { name, .. } => {
                let external_name = Self::external_type_label(name);
                quote!($external_name)
            }
            Type::Custom { name, .. } => {
                let custom_name = Self::struct_name(name);
                quote!($custom_name)
            }
            _ => todo!("Type::{:?} not implemented", ret_type),
        }
    }

    /// Get the native Dart FFI type rendering based on `Type`.
    pub fn native_type_label(native_ret_type: Option<&Type>) -> dart::Tokens {
        let Some(ret_type) = native_ret_type else {
            return quote!(Pointer<Void>);
        };
        match ret_type {
            Type::UInt8 => quote!(Uint8),
            Type::UInt16 => quote!(Uint16),
            Type::UInt32 => quote!(Uint32),
            Type::UInt64 => quote!(Uint64),
            Type::Int8 => quote!(Int8),
            Type::Int16 => quote!(Int16),
            Type::Int32 => quote!(Int32),
            Type::Int64 => quote!(Int64),
            Type::Float32 => quote!(Float),
            Type::Float64 => quote!(Double),
            Type::Boolean => quote!(Int8),
            Type::String => quote!(RustBuffer),
            Type::Bytes => quote!(RustBuffer),
            Type::Timestamp => quote!(RustBuffer),
            Type::Duration => quote!(RustBuffer),
            Type::Object { name, .. } => {
                let class_name = Self::struct_name(name);
                quote!(Pointer<$class_name>)
            }
            Type::Record { name, .. } => {
                let record_name = Self::struct_name(name);
                quote!(Pointer<$record_name>)
            }
            Type::Enum { name, .. } => {
                // Enums can often be represented as integers
                quote!(Int32)
            }
            Type::CallbackInterface { name, .. } => {
                let callback_name = Self::callback_name(name);
                quote!(Pointer<$callback_name>)
            }
            Type::Optional{inner_type, ..} => {
                let inner_label = Self::native_type_label(Some(inner_type.as_ref()));
                quote!(Pointer<$inner_label>)
            }
            Type::Sequence{inner_type, ..} => {
                let inner_label = Self::native_type_label(Some(inner_type.as_ref()));
                quote!(Pointer<$inner_label>)
            }
            Type::Map { key_type , value_type, .. } => {
                // Maps can be represented as Pointer<Void> or custom structs
                quote!(Pointer<Void>)
            }
            Type::External { name, .. } => {
                let external_label = Self::external_native_type_label(name);
                quote!($external_label)
            }
            Type::Custom { name, .. } => {
                let custom_name = Self::struct_name(name);
                quote!(Pointer<$custom_name>)
            }
            _ => todo!("Native Type::{:?} not implemented", ret_type),
        }
    }

    /// Get the native Dart FFI type rendering based on `Type`.
    pub fn native_dart_type_label(native_ret_type: Option<&Type>) -> dart::Tokens {
        let Some(ret_type) = native_ret_type else {
            return quote!(Pointer<Void>);
        };
        match ret_type {
            Type::UInt8
            | Type::UInt16
            | Type::UInt32
            | Type::UInt64
            | Type::Int8
            | Type::Int16
            | Type::Int32
            | Type::Int64 => quote!(int), // Adjust based on actual FFI size
            Type::Float32 => quote!(double),
            Type::Float64 => quote!(double),
            Type::Boolean => quote!(int),
            Type::String => quote!(RustBuffer),
            Type::Bytes => quote!(RustBuffer),
            Type::Timestamp => quote!(RustBuffer),
            Type::Duration => quote!(RustBuffer),
            Type::Object { name, .. } => {
                let class_name = Self::struct_name(name);
                quote!(Pointer<$class_name>)
            }
            Type::Record { name, .. } => {
                let record_name = Self::struct_name(name);
                quote!(Pointer<$record_name>)
            }
            Type::Enum { name, .. } => {
                // Enums can often be represented as integers
                quote!(Int32)
            }
            Type::CallbackInterface { name, .. } => {
                let callback_name = Self::callback_name(name);
                quote!(Pointer<$callback_name>)
            }
            Type::Optional{inner_type, ..} => {
                let inner_label = Self::native_type_label(Some(inner_type.as_ref()));
                quote!(Pointer<$inner_label>)
            }
            Type::Sequence{inner_type, ..} => {
                let inner_label = Self::native_type_label(Some(inner_type.as_ref()));
                quote!(Pointer<$inner_label>)
            }
            Type::Map { key_type , value_type, .. } => {
                // Maps can be represented as Pointer<Void> or custom structs
                quote!(Pointer<Void>)
            }
            Type::External { name, .. } => {
                let external_label = Self::external_native_type_label(name);
                quote!($external_label)
            }
            Type::Custom { name, .. } => {
                let custom_name = Self::struct_name(name);
                quote!(Pointer<$custom_name>)
            }
            _ => todo!("Native DartType::{:?} not implemented", ret_type),
        }
    }

    // Method to get the appropriate lift expression for callback arguments
    pub fn callback_arg_lift(arg_type: &Type, arg_name: &str) -> dart::Tokens {
        match arg_type {
            Type::Boolean => {
                // For booleans, we use direct comparison with 1
                quote!(final $arg_name = $arg_name == 1;)
            },
            Type::String => {
                // For strings, we use the string converter
                quote!(final $arg_name = FfiConverterString.lift($(arg_name)Buffer);)
            },
            Type::Optional { inner_type } => {
                if let Type::String = **inner_type {
                    // For optional strings
                    quote!(final $arg_name = FfiConverterOptionalString.lift($(arg_name)Buffer);)
                } else {
                    // For other optional types
                    let converter = arg_type.as_codetype().ffi_converter_name();
                    quote!(final $arg_name = $converter.lift($arg_name);)
                }
            },
            Type::Sequence { inner_type } => {
                if let Type::Int32 = **inner_type {
                    // For int32 sequences
                    quote!(final $arg_name = FfiConverterSequenceInt32.lift($(arg_name)Buffer);)
                } else {
                    // For other sequence types
                    let converter = arg_type.as_codetype().ffi_converter_name();
                    quote!(final $arg_name = $converter.lift($arg_name);)
                }
            },
            _ => {
                // For other types, use the standard lift function
                let converter = arg_type.as_codetype().ffi_converter_name();
                quote!(final $arg_name = $converter.lift($arg_name);)
            }
        }
    }

    // Method to get the appropriate callback parameter type
    pub fn callback_param_type(arg_type: &Type, arg_name: &str) -> dart::Tokens {
        match arg_type {
            Type::Boolean => quote!(int $arg_name),
            Type::String => quote!(RustBuffer $(arg_name)Buffer),
            Type::Optional { inner_type } => {
                if let Type::String = **inner_type {
                    quote!(RustBuffer $(arg_name)Buffer)
                } else {
                    let type_label = DartCodeOracle::dart_type_label(Some(arg_type));
                    quote!($type_label $arg_name)
                }
            },
            Type::Sequence { inner_type } => {
                if let Type::Int32 = **inner_type {
                    quote!(RustBuffer $(arg_name)Buffer)
                } else {
                    let type_label = DartCodeOracle::dart_type_label(Some(arg_type));
                    quote!($type_label $arg_name)
                }
            },
            _ => {
                let type_label = DartCodeOracle::dart_type_label(Some(arg_type));
                quote!($type_label $arg_name)
            }
        }
    }

    // Method to generate code for handling callback return values
    pub fn callback_return_handling(ret_type: &Type, method_name: &str, args: Vec<dart::Tokens>) -> dart::Tokens {
        match ret_type {
            Type::Boolean => {
                // For boolean return values
                quote!(
                    final result = obj.$method_name($(for arg in &args => $arg,));
                    outReturn.value = result ? 1 : 0;
                )
            },
            Type::Optional { inner_type } => {
                // For optional return values
                if let Type::String = **inner_type {
                    quote!(
                        final result = obj.$method_name($(for arg in &args => $arg,));
                        if (result == null) {
                            outReturn.ref = toRustBuffer(Uint8List.fromList([0]));
                        } else {
                            final lowered = FfiConverterOptionalString.lower(result);
                            outReturn.ref = toRustBuffer(lowered.asUint8List());
                        }
                    )
                } else {
                    let lowered = ret_type.as_codetype().ffi_converter_name();
                    quote!(
                        final result = obj.$method_name($(for arg in &args => $arg,));
                        if (result == null) {
                            outReturn.ref = toRustBuffer(Uint8List.fromList([0]));
                        } else {
                            final lowered = $lowered.lower(result);
                            final buffer = Uint8List(1 + lowered.length);
                            buffer[0] = 1;
                            buffer.setAll(1, lowered.asUint8List());
                            outReturn.ref = toRustBuffer(buffer);
                        }
                    )
                }
            },
            Type::String => {
                // For string return values
                quote!(
                    final result = obj.$method_name($(for arg in &args => $arg,));
                    outReturn.ref = FfiConverterString.lower(result);
                    status.code = CALL_SUCCESS;
                )
            },
            Type::Sequence { inner_type } => {
                if let Type::Int32 = **inner_type {
                    // For int32 sequence return values
                    quote!(
                        final result = obj.$method_name($(for arg in &args => $arg,));
                        outReturn.ref = FfiConverterSequenceInt32.lower(result);
                    )
                } else {
                    // For other sequence types
                    let lowered = ret_type.as_codetype().ffi_converter_name();
                    quote!(
                        final result = obj.$method_name($(for arg in &args => $arg,));
                        outReturn.ref = $lowered.lower(result);
                    )
                }
            },
            _ => {
                // For other return types
                let lowered = ret_type.as_codetype().ffi_converter_name();
                quote!(
                    final result = obj.$method_name($(for arg in &args => $arg,));
                    outReturn.ref = $lowered.lower(result);
                )
            }
        }
    }

    // Method to get the appropriate return type for callback functions
    pub fn callback_out_return_type(ret_type: Option<&Type>) -> dart::Tokens {
        if let Some(ret) = ret_type {
            match ret {
                Type::Boolean => quote!(Pointer<Int8>),
                _ => quote!(Pointer<RustBuffer>)
            }
        } else {
            quote!(Pointer<Void>)
        }
    }

    // Method to handle void return values in callbacks
    pub fn callback_void_handling(method_name: &str, args: Vec<dart::Tokens>) -> dart::Tokens {
        quote!(
            obj.$method_name($(for arg in &args => $arg,));
            status.code = CALL_SUCCESS;
        )
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
            Type::Record {name, .. } => Box::new(records::RecordCodeType::new(name)),
            Type::CallbackInterface { name, .. } => Box::new(callback_interface::CallbackInterfaceCodeType::new(name)),
            _ => todo!("As Type for Type::{:?}", self.as_type()),
        }
    }
}
