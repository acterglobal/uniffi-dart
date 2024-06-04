use std::{
    cell::RefCell,
    collections::{BTreeSet, HashMap},
};

use genco::prelude::*;
use uniffi_bindgen::{interface::Type, ComponentInterface};

use super::{enums, functions, objects, records};
use super::{
    render::{AsRenderable, Renderer, TypeHelperRenderer},
    Config,
};
use crate::gen::DartCodeOracle;

type FunctionDefinition = dart::Tokens;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ImportRequirement {
    Import { name: String },
    ImportAs { name: String, as_name: String },
}

// TODO: Handle importing external packages defined in the configuration.
// TODO: Finish refactor by moving all code that's not related to type helpers when Renderable has been implemented for the rest of the types
pub struct TypeHelpersRenderer<'a> {
    config: &'a Config,
    ci: &'a ComponentInterface,
    include_once_names: RefCell<HashMap<String, Type>>,
    imports: RefCell<BTreeSet<ImportRequirement>>,
}

#[allow(dead_code)]
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
        match self.config.external_packages.get(crate_name) {
            Some(name) => name.clone(),
            None => crate_name.to_string(),
        }
    }

    pub fn get_include_names(&self) -> HashMap<String, Type> {
        self.include_once_names.clone().into_inner()
    }
}

impl TypeHelperRenderer for TypeHelpersRenderer<'_> {
    // Checks if the type imports for each type have already been added
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

    fn get_record(&self, name: &str) -> Option<&uniffi_bindgen::interface::Record> {
        self.ci.get_record_definition(name)
    }

    fn get_ci(&self) -> &ComponentInterface {
        self.ci
    }
}

impl Renderer<(FunctionDefinition, dart::Tokens)> for TypeHelpersRenderer<'_> {
    // TODO: Implement a two pass system where the first pass will render the main code, and the second pass will render the helper code
    // this is so the generator knows what helper code to include.

    fn render(&self) -> (dart::Tokens, dart::Tokens) {
        // Render all the types and their helpers
        let types_definitions = quote! {
            $( for rec in self.ci.record_definitions() => $(records::generate_record(rec, self)))

            $( for enm in self.ci.enum_definitions() => $(enums::generate_enum(enm, self)))
            $( for obj in self.ci.object_definitions() => $(objects::generate_object(obj, self)))
        };

        // Render all the imports
        let imports: dart::Tokens = quote!();

        let function_definitions = quote!($(
            for fun in self.ci.function_definitions() => $(
                functions::generate_function("this", fun, self)
            ))
        );

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

                static UniffiInternalError unexpectedCall() {
                    return UniffiInternalError(unexpectedRustCallError, null);
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

            class LiftRetVal<T> {
                final T value;
                final int bytesRead;
                const LiftRetVal(this.value, this.bytesRead);

                LiftRetVal<T> copyWithOffset(int offset) {
                    return LiftRetVal(value, bytesRead + offset);
                }
            }


            class RustCallStatus extends Struct {
                @Int8()
                external int code;
                external RustBuffer errorBuf;

                static Pointer<RustCallStatus> allocate({int count = 1}) =>
                    calloc<RustCallStatus>(count * sizeOf<RustCallStatus>()).cast();
            }

            T noop<T>(T t) {
                return t;
            }

            T rustCall<T>(T Function(Pointer<RustCallStatus>) callback) {
                var callStatus = RustCallStatus.allocate();
                final returnValue = callback(callStatus);
                checkCallStatus(callStatus);
                return returnValue;
            }

            void checkCallStatus(Pointer<RustCallStatus> callStatus) {
                switch (callStatus.ref.code) {
                    case CALL_SUCCESS:
                        calloc.free(callStatus);
                        break;
                    case CALL_ERROR:
                        calloc.free(callStatus);
                        throw UniffiInternalError.unexpectedCall();
                    case CALL_PANIC:
                        if (callStatus.ref.errorBuf.len > 0) {
                            final message = utf8.decoder.convert(callStatus.ref.errorBuf.asUint8List());
                            calloc.free(callStatus);
                            throw UniffiInternalError.panicked(message);
                        } else {
                            calloc.free(callStatus);
                            throw UniffiInternalError.panicked("Rust panic");
                        }
                    default:
                        throw UniffiInternalError(callStatus.ref.code, null);
                    }
            }

            class RustBuffer extends Struct {
                @Uint64()
                external int capacity;

                @Uint64()
                external int len;

                external Pointer<Uint8> data;

                static RustBuffer fromBytes(Api api, ForeignBytes bytes) {
                    return rustCall((status) => $(DartCodeOracle::find_lib_instance()).$(self.ci.ffi_rustbuffer_from_bytes().name())(bytes, status));
                }

                // Needed so that the foreign language bindings can create buffers in which to pass complex data types across the FFI in the future
                static RustBuffer allocate(Api api, int size) {
                    return rustCall((status) => $(DartCodeOracle::find_lib_instance()).$(self.ci.ffi_rustbuffer_alloc().name())(size, status));
                }

                RustBuffer reserve(Api api, int additionalCapacity) {
                    return rustCall((status) => $(DartCodeOracle::find_lib_instance()).$(self.ci.ffi_rustbuffer_reserve().name())(this, additionalCapacity, status));
                }

                void free(Api api) {
                    rustCall((status) => $(DartCodeOracle::find_lib_instance()).$(self.ci.ffi_rustbuffer_free().name())(this, status));
                }


                Uint8List asUint8List() {
                    return data.cast<Uint8>().asTypedList(len);
                }

                @override
                String toString() {
                    return "RustBuffer { capacity: $capacity, len: $len, data: $data }";
                }
            }

            RustBuffer toRustBuffer(Api api, Uint8List data) {
                final length = data.length;

                final Pointer<Uint8> frameData = calloc<Uint8>(length); // Allocate a pointer large enough.
                final pointerList = frameData.asTypedList(length); // Create a list that uses our pointer and copy in the data.
                pointerList.setAll(0, data); // FIXME: can we remove this memcopy somehow?

                final bytes = calloc<ForeignBytes>();
                bytes.ref.len = length;
                bytes.ref.data = frameData;
                return RustBuffer.fromBytes(api, bytes.ref);
            }

            RustBuffer intToRustBuffer(Api api, int value) {
                int length = value.bitLength ~/ 8 + 1;

                // Ensure the length is either 4 or 8
                if (length != 4 && length != 8) {
                    length = (value < 0x100000000) ? 4 : 8;
                }


                final Pointer<Uint8> frameData = calloc<Uint8>(length); // Allocate a pointer large enough.
                final pointerList = frameData.asTypedList(length); // Create a list that uses our pointer and copy in the data.

                for (int i = length - 1; i >= 0; i--) {
                    pointerList[i] = value & 0xFF;
                    value >>= 8;
                }
                final bytes = calloc<ForeignBytes>();
                bytes.ref.len = length;
                bytes.ref.data = frameData;
                return RustBuffer.fromBytes(api, bytes.ref);
            }


            class ForeignBytes extends Struct {
                @Int32()
                external int len;

                external Pointer<Uint8> data;
            }


            $(helpers_definitions)

            $(types_definitions)

            const int UNIFFI_RUST_FUTURE_POLL_READY = 0;
            const int UNIFFI_RUST_FUTURE_POLL_MAYBE_READY = 1;

            typedef UniffiRustFutureContinuationCallback = Void Function(Uint64, Int8);

            Future<T> uniffiRustCallAsync<T, F>(
                int Function() rustFutureFunc,
                void Function(int, Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>, int) pollFunc,
                F Function(int, Pointer<RustCallStatus>) completeFunc,
                void Function(int) freeFunc,
                T Function(F) liftFunc,
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
                    final result = completeFunc(rustFuture, status);
                    checkCallStatus(status);
                    return liftFunc(result);
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
        | Type::Duration
        | Type::UInt64 => quote!(int),
        Type::Float32 | Type::Float64 => quote!(double),
        Type::String => quote!(String),
        Type::Object { name, .. } => quote!($name),
        Type::Boolean => quote!(bool),
        Type::Optional { inner_type } => quote!($(generate_type(inner_type))?),
        Type::Sequence { inner_type } => quote!(List<$(generate_type(inner_type))>),
        Type::Enum { name, .. } => quote!($name),
        // Type::Record { name,..  } => quote!($name),
        _ => todo!("Type::{:?}", ty), // AbiType::Num(ty) => self.generate_wrapped_num_type(*ty),
                                      // AbiType::Isize | AbiType::Usize => quote!(int),
                                      // AbiType::Bool => quote!(bool),
                                      // AbiType::RefStr | AbiType::String => quote!(String),
                                      // AbiType::RefSlice(ty) | AbiType::Vec(ty) => {
                                      //     quote!(List<#(self.generate_wrapped_num_type(*ty))>)
                                      // }
                                      // AbiType::Option(ty) => quote!(#(self.generate_type(ty))?),
                                      // AbiType::Result(ty) => self.generate_type(ty),
                                      // AbiType::Tuple(tuple) => match tuple.len() {
                                      //     0 => quote!(void),
                                      //     1 => self.generate_type(&tuple[0]),
                                      //     _ => quote!(List<dynamic>),
                                      // },
                                      // AbiType::RefObject(ty) | AbiType::Object(ty) => quote!(#ty),
                                      // AbiType::RefIter(ty) | AbiType::Iter(ty) => quote!(Iter<#(self.generate_type(ty))>),
                                      // AbiType::RefFuture(ty) | AbiType::Future(ty) => {
                                      //     quote!(Future<#(self.generate_type(ty))>)
                                      // }
                                      // AbiType::RefStream(ty) | AbiType::Stream(ty) => {
                                      //     quote!(Stream<#(self.generate_type(ty))>)
                                      // }
                                      // AbiType::Buffer(ty) => quote!(#(ffi_buffer_name_for(*ty))),
                                      // AbiType::List(ty) => quote!(#(format!("FfiList{}", ty))),
                                      // AbiType::RefEnum(ty) => quote!(#(ty)),
    }
}
