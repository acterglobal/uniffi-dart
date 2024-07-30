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
use crate::gen::DartCodeOracle;
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
    }}

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

        // let function_definitions = quote!($( for fun in self.ci.function_definitions() => $(functions::generate_function("this", fun, self))));

        let function_definitions = quote!(
            $(for fun in self.ci.function_definitions() => $(functions::generate_function(fun, self)))
        );

        // Let's include the string converter
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

            // class UniffiInternalError implements Exception {
            //     static const int bufferOverflow = 0;
            //     static const int incompleteData = 1;
            //     static const int unexpectedOptionalTag = 2;
            //     static const int unexpectedEnumCase = 3;
            //     static const int unexpectedNullPointer = 4;
            //     static const int unexpectedRustCallStatusCode = 5;
            //     static const int unexpectedRustCallError = 6;
            //     static const int unexpectedStaleHandle = 7;
            //     static const int rustPanic = 8;

            //     final int errorCode;
            //     final String? panicMessage;

            //     const UniffiInternalError(this.errorCode, this.panicMessage);

            //     static UniffiInternalError panicked(String message) {
            //     return UniffiInternalError(rustPanic, message);
            //     }

            //     @override
            //     String toString() {
            //     switch (errorCode) {
            //         case bufferOverflow:
            //         return "UniFfi::BufferOverflow";
            //         case incompleteData:
            //         return "UniFfi::IncompleteData";
            //         case unexpectedOptionalTag:
            //         return "UniFfi::UnexpectedOptionalTag";
            //         case unexpectedEnumCase:
            //         return "UniFfi::UnexpectedEnumCase";
            //         case unexpectedNullPointer:
            //         return "UniFfi::UnexpectedNullPointer";
            //         case unexpectedRustCallStatusCode:
            //         return "UniFfi::UnexpectedRustCallStatusCode";
            //         case unexpectedRustCallError:
            //         return "UniFfi::UnexpectedRustCallError";
            //         case unexpectedStaleHandle:
            //         return "UniFfi::UnexpectedStaleHandle";
            //         case rustPanic:
            //         return "UniFfi::rustPanic: $$panicMessage";
            //         default:
            //         return "UniFfi::UnknownError: $$errorCode";
            //     }
            //     }
            // }

            // const int CALL_SUCCESS = 0;
            // const int CALL_ERROR = 1;
            // const int CALL_PANIC = 2;

            // class RustCallStatus extends Struct {
            //     @Int8()
            //     external int code;
            //     external RustBuffer errorBuf;

            //     static Pointer<RustCallStatus> allocate({int count = 1}) =>
            //     calloc<RustCallStatus>(count * sizeOf<RustCallStatus>()).cast();
            // }

            // T noop<T>(T t) {
            //     return t;
            // }

            // T rustCall<T>(T Function(Pointer<RustCallStatus>) callback) {
            //     var callStatus = RustCallStatus.allocate();
            //     final returnValue = callback(callStatus);

            //     switch (callStatus.ref.code) {
            //     case CALL_SUCCESS:
            //         calloc.free(callStatus);
            //         return returnValue;
            //     case CALL_ERROR:
            //         throw callStatus.ref.errorBuf;
            //     case CALL_PANIC:
            //         if (callStatus.ref.errorBuf.len > 0) {
            //             final message = liftString(callStatus.ref.errorBuf.asUint8List());
            //             calloc.free(callStatus);
            //             throw UniffiInternalError.panicked(message);
            //         } else {
            //             calloc.free(callStatus);
            //             throw UniffiInternalError.panicked("Rust panic");
            //         }
            //     default:
            //         throw UniffiInternalError(callStatus.ref.code, null);
            //     }
            // }

            // class RustBuffer extends Struct {
            //     @Uint64()
            //     external int capacity;

            //     @Uint64()
            //     external int len;

            //     external Pointer<Uint8> data;

            //     static RustBuffer fromBytes(ForeignBytes bytes) {
            //         final _fromBytesPtr = api._lookup<
            //         NativeFunction<
            //             RustBuffer Function(ForeignBytes, Pointer<RustCallStatus>)>>($(format!("\"{}\"", self.ci.ffi_rustbuffer_from_bytes().name())));
            //         final _fromBytes =
            //         _fromBytesPtr.asFunction<RustBuffer Function(ForeignBytes, Pointer<RustCallStatus>)>();
            //         return rustCall((res) => _fromBytes(bytes, res));
            //     }

            //     // Needed so that the foreign language bindings can create buffers in which to pass complex data types across the FFI in the future
            //     static RustBuffer allocate(int size) {
            //         final _allocatePtr = api._lookup<
            //             NativeFunction<
            //                 RustBuffer Function(Int64, Pointer<RustCallStatus>)>>($(format!("\"{}\"", self.ci.ffi_rustbuffer_alloc().name())));
            //         final _allocate = _allocatePtr.asFunction<RustBuffer Function(int, Pointer<RustCallStatus>)>();
            //         return rustCall((res) => _allocate(size, res));
            //     }

            //     void deallocate() {
            //         final _freePtr = api._lookup<
            //         NativeFunction<
            //             Void Function(RustBuffer, Pointer<RustCallStatus>)>>($(format!("\"{}\"", self.ci.ffi_rustbuffer_free().name())));
            //         final _free = _freePtr.asFunction<void Function(RustBuffer, Pointer<RustCallStatus>)>();
            //         rustCall((res) => _free(this, res));
            //     }

            //     Uint8List asUint8List() {
            //         final buf = Uint8List(len);
            //         final precast = data.cast<Uint8>();
            //         for (int i = 0; i < len; i++) {
            //             buf[i] = precast.elementAt(i).value;
            //         }
            //         return buf;
            //     }

            //     @override
            //     String toString() {
            //         String res = "RustBuffer { capacity: $capacity, len: $len, data: $data }";
            //         final precast = data.cast<Uint8>();
            //         for (int i = 0; i < len; i++) {
            //             int char = precast.elementAt(i).value;
            //             res += String.fromCharCode(char);
            //         }
            //         return res;
            //     }
            // }

            // // TODO: Make all the types use me!
            // abstract class FfiConverter<T, V> {
            //     T lift(V value, [int offset]);
            //     V lower(T value);
            //     T read(ByteBuffer buf);
            //     int size([T value]);
            //     void write(T value, ByteBuffer buf);

            //     RustBuffer lowerIntoRustBuffer(T value) {
            //       throw UnimplementedError("lower into rust implement lift from rust buffer");
            //       // final rbuf = RustBuffer.allocate(size());
            //       // try {
            //       //   final bbuf = rbuf.data.asByteBuffer(0, rbuf.capacity);
            //       //   write(value, bbuf);
            //       //   rbuf.len = bbuf.position();
            //       //   return rbuf;
            //       // } catch (e) {
            //       //   RustBuffer.deallocate(rbuf);
            //       //   throw e;
            //       // }
            //     }

            //     T liftFromRustBuffer(RustBuffer rbuf) {
            //       throw UnimplementedError("Lift from rust implement lift from rust buffer");
            //       // final byteBuf = rbuf.asByteBuffer();
            //       // try {
            //       //   final item = read(byteBuf);
            //       //   if (byteBuf.hasRemaining) {
            //       //     throw Exception(
            //       //         "Junk remaining in buffer after lifting, something is very wrong!!");
            //       //   }
            //       //   return item;
            //       // } finally {
            //       //   RustBuffer.deallocate(rbuf);
            //       // }
            //     }
            // }

            // abstract class FfiConverterRustBuffer<T>
            //       implements FfiConverter<T, RustBuffer> {
            //     @override
            //     T lift(RustBuffer value, [int offset = 0]) => this.liftFromRustBuffer(value);
            //     @override
            //     RustBuffer lower(T value) => this.lowerIntoRustBuffer(value);
            // }

            // String liftString(Uint8List input) {
            //     // we have a i32 length at the front
            //     return utf8.decoder.convert(input);
            // }


            // Uint8List lowerString(String input) {
            //     // FIXME: this is too many memcopies!
            //     return Utf8Encoder().convert(input);
            // }


            // RustBuffer toRustBuffer(Uint8List data) {
            //     final length = data.length;

            //     final Pointer<Uint8> frameData = calloc<Uint8>(length); // Allocate a pointer large enough.
            //     final pointerList = frameData.asTypedList(length); // Create a list that uses our pointer and copy in the data.
            //     pointerList.setAll(0, data); // FIXME: can we remove this memcopy somehow?

            //     final bytes = calloc<ForeignBytes>();
            //     bytes.ref.len = length;
            //     bytes.ref.data = frameData;
            //     return RustBuffer.fromBytes(bytes.ref);
            // }

            // T? liftOptional<T>(Uint8List buf, T? Function(Uint8List) lifter) {
            //     if (buf.isEmpty || buf.first == 0){
            //         return null;
            //     }
            //     return lifter(buf);
            // }

            //$(primitives::generate_wrapper_lifters())


            // Uint8List lowerOptional<T>(T? inp, Uint8List Function(T) lowerer) {
            //     if (inp == null) {
            //         final res = Uint8List(1);
            //         res.first = 0;
            //         return res;
            //     }
            //     // converting the inner
            //     final inner = lowerer(inp);
            //     // preparing the outer
            //     final offset = 5;
            //     final res = Uint8List(inner.length + offset);
            //     // first byte sets the option to as true
            //     res.setAll(0, [1]);
            //     // then set the inner size
            //     final len = Uint32List(1);
            //     len.first = inner.length;
            //     res.setAll(1, len.buffer.asUint8List().reversed);
            //     // then add the actual data
            //     res.setAll(offset, inner);
            //     return res;
            // }

            // $(primitives::generate_primitives_lowerers())
            // $(primitives::generate_primitives_lifters())
            // $(primitives::generate_wrapper_lowerers())

            // class ForeignBytes extends Struct {
            //     @Int32()
            //     external int len;

            //     external Pointer<Uint8> data;
            // }


            $(types_definitions)


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
            const int CALL_UNEXPECTED_ERROR = 2;

            class RustCallStatus extends Struct {
                @Int8()
                external int code;

                external RustBuffer errorBuf;

                //Pointer<RustCallStatus> asPointer() => Pointer<RustCallStatus>.fromAddress(address);
            }

            void checkCallStatus(UniffiRustCallStatusErrorHandler errorHandler, RustCallStatus status) {

                if (status.code == CALL_SUCCESS) {
                return;
                } else if (status.code == CALL_ERROR) {
                throw errorHandler.lift(status.errorBuf);
                } else if (status.code == CALL_UNEXPECTED_ERROR) {
                if (status.errorBuf.len > 0) {
                    throw UniffiInternalError.panicked(FfiConverterString.lift(status.errorBuf));
                } else {
                    throw UniffiInternalError.panicked("Rust panic");
                }
                } else {
                throw UniffiInternalError.panicked("Unexpected RustCallStatus code: ${status.code}");
                }
            }

            T rustCall<T>(T Function(Pointer<RustCallStatus>) callback) {
                final status = calloc<RustCallStatus>();
                try {
                return callback(status);
                } finally {
                calloc.free(status);
                }
            }

            class NullRustCallStatusErrorHandler extends UniffiRustCallStatusErrorHandler {
                @override
                Exception lift(RustBuffer errorBuf) {
                errorBuf.free();
                return UniffiInternalError.panicked("Unexpected CALL_ERROR");
                }
            }

            abstract class UniffiRustCallStatusErrorHandler {
                Exception lift(RustBuffer errorBuf);
            }

            class RustBuffer extends Struct {
                @Uint64()
                external int capacity;

                @Uint64()
                external int len;

                external Pointer<Uint8> data;

                static RustBuffer alloc(int size) {
                    return rustCall((status) => $(DartCodeOracle::find_lib_instance()).$(self.ci.ffi_rustbuffer_alloc().name())(size, status));
                }

                static RustBuffer fromBytes(ForeignBytes bytes) {
                    return rustCall((status) => $(DartCodeOracle::find_lib_instance()).$(self.ci.ffi_rustbuffer_from_bytes().name())(bytes, status));
                }

                // static RustBuffer from(Pointer<Uint8> bytes, int len) {
                //   final foreignBytes = ForeignBytes(len: len, data: bytes);
                //   return rustCall((status) => _UniffiLib.instance.ffi_uniffi_futures_rustbuffer_from_bytes(foreignBytes));
                // }

                void free() {
                    rustCall((status) => $(DartCodeOracle::find_lib_instance()).$(self.ci.ffi_rustbuffer_free().name())(this, status));
                }

                RustBuffer reserve(int additionalCapacity) {
                return rustCall((status) => $(DartCodeOracle::find_lib_instance()).$(self.ci.ffi_rustbuffer_reserve().name())(this, additionalCapacity, status));
                }

                Uint8List asUint8List() {
                final dataList = data.asTypedList(len);
                final byteData = ByteData.sublistView(dataList);
                return Uint8List.view(byteData.buffer);
                }

                @override
                String toString() {
                return "RustBuffer{capacity: $capacity, len: $len, data: $data}";
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

                //ForeignBytes({required this.len, required this.data});

                // factory ForeignBytes.fromTypedData(Uint8List typedData) {
                //   final data = calloc<Uint8>(typedData.length);
                //   final dataList = data.asTypedList(typedData.length);
                //   dataList.setAll(0, typedData);
                //   return ForeignBytes(len: typedData.length, data: data);
                // }

                void free() {
                calloc.free(data);
                }
            }

            class LiftRetVal<T> {
                final T value;
                final int bytesRead;
                const LiftRetVal(this.value, this.bytesRead);

                LiftRetVal<T> copyWithOffset(int offset) {
                    return LiftRetVal(value, bytesRead + offset);
                }
            }

            abstract class FfiConverter<D, F> {
                const FfiConverter();

                D lift(F value);
                F lower(D value);
                D read(ByteData buffer, int offset);
                void write(D value, ByteData buffer, int offset);
                int size(D value);
            }

            mixin FfiConverterPrimitive<T> on FfiConverter<T, T> {
                @override
                T lift(T value) => value;

                @override
                T lower(T value) => value;
            }

            Uint8List createUint8ListFromInt(int value) {
                int length = value.bitLength ~/ 8 + 1;

                // Ensure the length is either 4 or 8
                if (length != 4 && length != 8) {
                length = (value < 0x100000000) ? 4 : 8;
                }

                Uint8List uint8List = Uint8List(length);

                for (int i = length - 1; i >= 0; i--) {
                uint8List[i] = value & 0xFF;
                value >>= 8;
                }

                return uint8List;
            }

                LiftRetVal<T> copyWithOffset(int offset) {
                    return LiftRetVal(value, bytesRead + offset);
                }
            }

            const int UNIFFI_RUST_FUTURE_POLL_READY = 0;
            const int UNIFFI_RUST_FUTURE_POLL_MAYBE_READY = 1;

            typedef UniffiRustFutureContinuationCallback = Void Function(Uint64, Int8);

            Future<T> uniffiRustCallAsync<T, F>(
                int Function() rustFutureFunc,
                void Function(int, Pointer<NativeFunction<UniffiRustFutureContinuationCallback>>, int) pollFunc,
                F Function(int, Pointer<RustCallStatus>) completeFunc,
                void Function(int) freeFunc,
                T Function(F) liftFunc, [
                UniffiRustCallStatusErrorHandler? errorHandler,
            ]) async {
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
                      
                        // checkCallStatus(errorHandler ?? NullRustCallStatusErrorHandler(), status.ref);
                       
                        return liftFunc(result);
                    } finally {
                        calloc.free(status);
                    }
                } finally {
                    freeFunc(rustFuture);
                }
            }

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

// pub fn generate_ffi_type(ret: Option<&FfiType>) -> dart::Tokens {
//     let Some(ret_type) = ret else {
//         return quote!(Void)
//     };
//     match *ret_type {
//         FfiType::UInt8 => quote!(Uint8),
//         FfiType::UInt16 => quote!(Uint16),
//         FfiType::UInt32 => quote!(Uint32),
//         FfiType::UInt64 => quote!(Uint64),
//         FfiType::Int8 => quote!(Int8),
//         FfiType::Int16 => quote!(Int16),
//         FfiType::Int32 => quote!(Int32),
//         FfiType::Int64 => quote!(Int64),
//         FfiType::Float32 => quote!(Float),
//         FfiType::Float64 => quote!(Double),
//         FfiType::RustBuffer(ref inner) => match inner {
//             Some(i) => quote!($i),
//             _ => quote!(RustBuffer),
//         },
//         FfiType::RustArcPtr(_) => quote!(Pointer<Void>),
//         _ => todo!("FfiType::{:?}", ret_type),
//     }
// }

// pub fn generate_ffi_dart_type(ret: Option<&FfiType>) -> dart::Tokens {
//     let Some(ret_type) = ret else {
//         return quote!(void)
//     };
//     match *ret_type {
//         FfiType::UInt8 => quote!(int),
//         FfiType::UInt16 => quote!(int),
//         FfiType::UInt32 => quote!(int),
//         FfiType::UInt64 => quote!(int),
//         FfiType::Int8 => quote!(int),
//         FfiType::Int16 => quote!(int),
//         FfiType::Int32 => quote!(int),
//         FfiType::Int64 => quote!(int),
//         FfiType::Float32 | FfiType::Float64 => quote!(double),
//         FfiType::RustBuffer(ref inner) => match inner {
//             Some(i) => quote!($i),
//             _ => quote!(RustBuffer),
//         },
//         FfiType::RustArcPtr(_) => quote!(Pointer<Void>),
//         //FfiType::ForeignExecutorHandle => ,
//         _ => todo!("FfiType::{:?}", ret_type),
//     }
// }

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