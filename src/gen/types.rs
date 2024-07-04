use std::{
    cell::RefCell,
    collections::{BTreeSet, HashMap},
};

use genco::prelude::*;
use uniffi_bindgen::{
    interface::{FfiType, Type},
    ComponentInterface,
};

use super::{enums, functions, objects, oracle::AsCodeType, primitives, records};
use crate::gen::oracle::DartCodeOracle;
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

    pub fn get_include_names(&self) -> HashMap<String, Type> {
        self.include_once_names.clone().into_inner()
    }
}

impl TypeHelperRenderer for TypeHelpersRenderer<'_> {
    fn include_once_check(&self, name: &str, ty: &Type) -> bool {
        let mut map = self.include_once_names.borrow_mut();
        let found = map.insert(name.to_string(), ty.clone()).is_some();
        drop(map);
        found
    }

    fn check(&self, name: &str) -> bool {
        let map = self.include_once_names.borrow();
        let contains = map.contains_key(&name.to_string());
        drop(map);
        contains
    }

    fn add_import(&self, name: &str) -> bool {
        self.imports.borrow_mut().insert(ImportRequirement::Import {
            name: name.to_owned(),
        })
    }

    fn add_import_as(&self, name: &str, as_name: &str) -> bool {
        self.imports
            .borrow_mut()
            .insert(ImportRequirement::ImportAs {
                name: name.to_owned(),
                as_name: as_name.to_owned(),
            })
    }

    fn get_object(&self, name: &str) -> Option<&uniffi_bindgen::interface::Object> {
        self.ci.get_object_definition(name)
    }

    fn get_enum(&self, name: &str) -> Option<&uniffi_bindgen::interface::Enum> {
        self.ci.get_enum_definition(name)
    }

    fn get_ci(&self) -> &ComponentInterface {
        self.ci
    }

    fn get_record(&self, name: &str) -> Option<&uniffi_bindgen::interface::Record> {
        self.ci.get_record_definition(name)
    }

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

impl Renderer<(FunctionDefinition, dart::Tokens)> for TypeHelpersRenderer<'_> {
    fn render(&self) -> (dart::Tokens, dart::Tokens) {
        let types_definitions = quote! {
            $( for rec in self.ci.record_definitions() => $(records::generate_record(rec, self)))

            $( for enm in self.ci.enum_definitions() => $(enums::generate_enum(enm, self)))
            $( for obj in self.ci.object_definitions() => $(objects::generate_object(obj, self)))
        };

        let imports: dart::Tokens = quote!();

        let function_definitions = quote!(
            $(for fun in self.ci.function_definitions() => $(functions::generate_function(fun, self)))
        );

        self.include_once_check(&Type::String.as_codetype().canonical_name(), &Type::String);
        let helpers_definitions = quote! {
            $(for (_, ty) in self.get_include_names().iter() => $(ty.as_renderable().render_type_helper(self)) )
        };

        let types_helper_code = quote! {
            import "dart:async";
            import "dart:convert";
            import "dart:ffi";
            import "dart:io" show Platform, File, Directory;
            import "dart:isolate";
            import "dart:typed_data";
            import "package:ffi/ffi.dart";
            $(imports)

            class UniffiInternalError implements Exception {
                static const int bufferOverflow = 0;
                static const int incompleteData = 1;
                static const int unexpectedOptionalTag = 2;
                static const int unexpectedEnumCase = 3;
                static const int unexpectedNullPointer = 4;
                static const int unexpectedRustCallStatusCode = 5;
                static const int unexpectedRustCallError = 6;
                static const int unexpectedStaleHandle = 7;
                static const int rustPanic = 8;

                final int errorCode;
                final String? panicMessage;

                const UniffiInternalError(this.errorCode, this.panicMessage);

                static UniffiInternalError panicked(String message) {
                return UniffiInternalError(rustPanic, message);
                }

                @override
                String toString() {
                switch (errorCode) {
                    case bufferOverflow:
                    return "UniFfi::BufferOverflow";
                    case incompleteData:
                    return "UniFfi::IncompleteData";
                    case unexpectedOptionalTag:
                    return "UniFfi::UnexpectedOptionalTag";
                    case unexpectedEnumCase:
                    return "UniFfi::UnexpectedEnumCase";
                    case unexpectedNullPointer:
                    return "UniFfi::UnexpectedNullPointer";
                    case unexpectedRustCallStatusCode:
                    return "UniFfi::UnexpectedRustCallStatusCode";
                    case unexpectedRustCallError:
                    return "UniFfi::UnexpectedRustCallError";
                    case unexpectedStaleHandle:
                    return "UniFfi::UnexpectedStaleHandle";
                    case rustPanic:
                    return "UniFfi::rustPanic: $$panicMessage";
                    default:
                    return "UniFfi::UnknownError: $$errorCode";
                }
                }
            }

            const int CALL_SUCCESS = 0;
            const int CALL_ERROR = 1;
            const int CALL_PANIC = 2;

            class RustCallStatus extends Struct {
                @Int8()
                external int code;
                external RustBuffer errorBuf;

                static Pointer<RustCallStatus> allocate({int count = 1}) =>
                calloc<RustCallStatus>(count * sizeOf<RustCallStatus>()).cast();
            }

            T rustCall<T>(T Function(Pointer<RustCallStatus>) callback) {
                var callStatus = RustCallStatus.allocate();
                try {
                    final returnValue = callback(callStatus);

                    switch (callStatus.ref.code) {
                    case CALL_SUCCESS:
                        return returnValue;
                    case CALL_ERROR:
                        throw callStatus.ref.errorBuf;
                    case CALL_PANIC:
                        if (callStatus.ref.errorBuf.len > 0) {
                            final message = utf8.decode(callStatus.ref.errorBuf.asUint8List());
                            throw UniffiInternalError.panicked(message);
                        } else {
                            throw UniffiInternalError.panicked("Rust panic");
                        }
                    default:
                        throw UniffiInternalError(callStatus.ref.code, null);
                    }
                } finally {
                    calloc.free(callStatus);
                }
            }

            class RustBuffer extends Struct {
                @Uint64()
                external int capacity;

                @Uint64()
                external int len;

                external Pointer<Uint8> data;

                static RustBuffer fromBytes(ForeignBytes bytes) {
                    return rustCall((res) => _UniffiLib.instance.$(self.ci.ffi_rustbuffer_from_bytes().name())(bytes, res));
                }

                static RustBuffer allocate(int size) {
                    return rustCall((res) => _UniffiLib.instance.$(self.ci.ffi_rustbuffer_alloc().name())(size, res));
                }

                void free() {
                    rustCall((res) => _UniffiLib.instance.$(self.ci.ffi_rustbuffer_free().name())(this, res));
                }

                Uint8List asUint8List() {
                    return data.cast<Uint8>().asTypedList(len);
                }

                @override
                String toString() {
                    return "RustBuffer { capacity: $capacity, len: $len, data: $data }";
                }
            }

            RustBuffer toRustBuffer(Uint8List data) {
                final length = data.length;

                final Pointer<Uint8> frameData = calloc<Uint8>(length);
                final pointerList = frameData.asTypedList(length);
                pointerList.setAll(0, data);

                final bytes = calloc<ForeignBytes>();
                bytes.ref.len = length;
                bytes.ref.data = frameData;
                return RustBuffer.fromBytes(bytes.ref);
            }

            class ForeignBytes extends Struct {
                @Int32()
                external int len;
                external Pointer<Uint8> data;
            }

            class LiftRetVal<T> {
                final T value;
                final int bytesRead;
                const LiftRetVal(this.value, this.bytesRead);

                LiftRetVal<T> copyWithOffset(int offset) {
                    return LiftRetVal(value, bytesRead + offset);
                }
            }

            $(types_definitions)

            $(helpers_definitions)

            const int UNIFFI_RUST_FUTURE_POLL_READY = 0;
            const int UNIFFI_RUST_FUTURE_POLL_MAYBE_READY = 1;

            typedef UniffiRustFutureContinuationCallback = Void Function(Uint64, Int8);

            Future<T> uniffiRustCallAsync<T, F>(
                int Function() rustFutureFunc,
                void Function(int, Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>, int) pollFunc,
                F Function(int, Pointer<RustCallStatus>) completeFunc,
                void Function(int) freeFunc,
                T Function(F) liftFunc
            ) async {
                final rustFuture = rustFutureFunc();
                final completer = Completer<int>();

                late final NativeCallable<UniffiRustFutureContinuationCallback> callback;

                void poll() {
                    pollFunc(
                        rustFuture,
                        callback.nativeFunction,
                        0,
                    );
                }
                void onResponse(int _idx, int pollResult) {
                    if (pollResult == UNIFFI_RUST_FUTURE_POLL_READY) {
                        completer.complete(pollResult);
                    } else {
                        poll();
                    }
                }
                callback = NativeCallable<UniffiRustFutureContinuationCallback>.listener(onResponse);

                try {
                    poll();
                    await completer.future;
                    callback.close();

                    final status = calloc<RustCallStatus>();
                    try {
                        final result = completeFunc(rustFuture, status);
                        return liftFunc(result);
                    } finally {
                        calloc.free(status);
                    }
                } finally {
                    freeFunc(rustFuture);
                }
            }
        };

        (types_helper_code, function_definitions)
    }
}

pub fn generate_type(ty: &Type) -> dart::Tokens {
    match ty {
        Type::UInt8
        | Type::UInt32
        | Type::Int8
        | Type::Int16
        | Type::Int64
        | Type::UInt16
        | Type::Int32
        | Type::UInt64 => quote!(int),
        Type::Float32 | Type::Float64 => quote!(double),
        Type::String => quote!(String),
        Type::Object { name, .. } => quote!($name),
        Type::Boolean => quote!(bool),
        Type::Optional { inner_type } => quote!($(generate_type(inner_type))?),
        Type::Sequence { inner_type } => quote!(List<$(generate_type(inner_type))>),
        Type::Enum { name, .. } => quote!($name),
        Type::Duration => quote!(Duration),
        _ => todo!("Type::{:?}", ty),
    }
}

