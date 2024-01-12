use std::{cell::RefCell, collections::{HashSet, BTreeSet}};

use genco::prelude::*;
use uniffi_bindgen::{interface::{FfiType, Type}, ComponentInterface};

use super::{Config, render::{Renderable, Renderer}};
use crate::gen::render::primitives;


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
    include_once_names: RefCell<HashSet<String>>,
    imports: RefCell<BTreeSet<ImportRequirement>>,
}

impl<'a> TypeHelpersRenderer<'a> {
    pub fn new(config: &'a Config, ci: &'a ComponentInterface) -> Self {
        Self {
            config,
            ci,
            include_once_names: RefCell::new(HashSet::new()),
            imports: RefCell::new(BTreeSet::new()),
        }
    }

    fn external_type_package_name(&self, crate_name: &str) -> String {
        match self.config.external_packages.get(crate_name) {
            Some(name) => name.clone(),
            None => crate_name.to_string(),
        }
    }

    fn include_once_check(&self, name: &str) -> bool {
        self.include_once_names
            .borrow_mut()
            .insert(name.to_string())
    }

    fn add_import(&self, name: &str) -> &str {
        self.imports.borrow_mut().insert(ImportRequirement::Import {
            name: name.to_owned(),
        });
        ""
    }

    fn add_import_as(&self, name: &str, as_name: &str) -> &str {
        self.imports
            .borrow_mut()
            .insert(ImportRequirement::ImportAs {
                name: name.to_owned(),
                as_name: as_name.to_owned(),
            });
        ""
    }
}

impl Renderable for TypeHelpersRenderer<'_> {
    fn render_type(&self, ty: &uniffi_bindgen::interface::Type) -> dart::Tokens {
        todo!()
    }

    fn render_type_helpers(&self, ty: &uniffi_bindgen::interface::Type) -> dart::Tokens {
        todo!()
    }
}

impl Renderer for TypeHelpersRenderer<'_> {
    fn render(&self, _r: &impl Renderable) -> dart::Tokens {
        quote! {
            import "dart:async";
            import "dart:convert";
            import "dart:ffi";
            import "dart:io" show Platform, File, Directory;
            import "dart:isolate";
            import "dart:typed_data";
            import "package:ffi/ffi.dart";


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

            T noop<T>(T t) {
                return t;
            }

            T rustCall<T>(Api api, T Function(Pointer<RustCallStatus>) callback) {
                var callStatus = RustCallStatus.allocate();
                final returnValue = callback(callStatus);

                switch (callStatus.ref.code) {
                case CALL_SUCCESS:
                    calloc.free(callStatus);
                    return returnValue;
                case CALL_ERROR:
                    throw callStatus.ref.errorBuf;
                case CALL_PANIC:
                    if (callStatus.ref.errorBuf.len > 0) {
                        final message = liftString(api, callStatus.ref.errorBuf.toIntList());
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
                @Int32()
                external int capacity;

                @Int32()
                external int len;

                external Pointer<Uint8> data;

                static RustBuffer fromBytes(Api api, ForeignBytes bytes) {
                    final _fromBytesPtr = api._lookup<
                    NativeFunction<
                        RustBuffer Function(ForeignBytes, Pointer<RustCallStatus>)>>($(format!("\"{}\"", self.ci.ffi_rustbuffer_from_bytes().name())));
                    final fromBytes =
                    _fromBytesPtr.asFunction<RustBuffer Function(ForeignBytes, Pointer<RustCallStatus>)>();
                    return rustCall(api, (res) => fromBytes(bytes, res));
                }

                // Needed so that the foreign language bindings can create buffers in which to pass complex data types across the FFI in the future
                static RustBuffer allocate(Api api, int size) {
                    final _allocatePtr = api._lookup<
                        NativeFunction<
                            RustBuffer Function(Int32, Pointer<RustCallStatus>)>>($(format!("\"{}\"", self.ci.ffi_rustbuffer_alloc().name())));
                    final allocate = _allocatePtr.asFunction<RustBuffer Function(int, Pointer<RustCallStatus>)>();
                    return rustCall(api, (res) => allocate(size, res));
                }

                void deallocate(Api api) {
                    final _freePtr = api._lookup<
                    NativeFunction<
                        Void Function(RustBuffer, Pointer<RustCallStatus>)>>($(format!("\"{}\"", self.ci.ffi_rustbuffer_free().name())));
                    final free = _freePtr.asFunction<void Function(RustBuffer, Pointer<RustCallStatus>)>();
                    rustCall(api, (res) => free(this, res));
                }

                Uint8List toIntList() {
                    final buf = Uint8List(len);
                    final precast = data.cast<Uint8>();
                    for (int i = 0; i < len; i++) {
                        buf[i] = precast.elementAt(i).value;
                    }
                    return buf;
                }

                @override
                String toString() {
                    String res = "RustBuffer { capacity: $capacity, len: $len, data: $data }";
                    final precast = data.cast<Uint8>();
                    for (int i = 0; i < len; i++) {
                        int char = precast.elementAt(i).value;
                        res += String.fromCharCode(char);
                    }
                    return res;
                }
            }

            // TODO: Make all the types use me!
            abstract class FfiConverter<T, FfiType> {
                T lift(Api api, FfiType value);
                FfiType lower(Api api,T value);
                T read(ByteBuffer buf);
                int allocationSize(T value);
                void write(T value, ByteBuffer buf);
              
                RustBuffer lowerIntoRustBuffer(Api api, T value) {
                  throw UnimplementedError("lower into rust implement lift from rust buffer");
                  // final rbuf = RustBuffer.allocate(api, allocationSize(value));
                  // try {
                  //   final bbuf = rbuf.data.asByteBuffer(0, rbuf.capacity);
                  //   write(value, bbuf);
                  //   rbuf.len = bbuf.position();
                  //   return rbuf;
                  // } catch (e) {
                  //   RustBuffer.deallocate(api, rbuf);
                  //   throw e;
                  // }
                }
              
                T liftFromRustBuffer(Api api, RustBuffer rbuf) {
                  throw UnimplementedError("Lift from rust implement lift from rust buffer");
                  // final byteBuf = rbuf.asByteBuffer();
                  // try {
                  //   final item = read(byteBuf);
                  //   if (byteBuf.hasRemaining) {
                  //     throw Exception(
                  //         "Junk remaining in buffer after lifting, something is very wrong!!");
                  //   }
                  //   return item;
                  // } finally {
                  //   RustBuffer.deallocate(rbuf);
                  // }
                }
              }
              
              abstract class FfiConverterRustBuffer<T>
                  implements FfiConverter<T, RustBuffer> {
                @override
                T lift(Api api, RustBuffer value) => liftFromRustBuffer(api, value);
                @override
                RustBuffer lower(Api api, T value) => lowerIntoRustBuffer(api, value);
              }

            String liftString(Api api, Uint8List input) {        
                // we have a i32 length at the front
                return utf8.decoder.convert(input);
            }

            $(primitives::generate_primitives_lifters())
           
            Uint8List lowerString(Api api, String input) {
                // FIXME: this is too many memcopies!
                return Utf8Encoder().convert(input);
            }

            $(primitives::generate_primitives_lowerers())

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

            T? liftOptional<T>(Api api, Uint8List buf, T? Function(Api, Uint8List) lifter) {
                if (buf.isEmpty || buf.first == 0){
                    return null;
                }
                return lifter(api, buf);
            }

            $(primitives::generate_wrapper_lifters())

            Uint8List lowerOptional<T>(Api api, T? inp, Uint8List Function(Api, T) lowerer) {
                if (inp == null) {
                    final res = Uint8List(1);
                    res.first = 0;
                    return res;
                }
                // converting the inner
                final inner = lowerer(api, inp);
                // preparing the outer
                final offset = 5;
                final res = Uint8List(inner.length + offset);
                // first byte sets the option to as true
                res.setAll(0, [1]);
                // then set the inner size
                final len = Uint32List(1);
                len.first = inner.length;
                res.setAll(1, len.buffer.asUint8List().reversed);
                // then add the actual data
                res.setAll(offset, inner);
                return res;
            }

            $(primitives::generate_wrapper_lowerers())

            class ForeignBytes extends Struct {
                @Int32()
                external int len;

                external Pointer<Uint8> data;
            }
        }
    }
}


pub fn generate_ffi_type(ret: Option<&FfiType>) -> dart::Tokens {
    let Some(ret_type) = ret else {
        return quote!(Void)
    };
    match *ret_type {
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
        FfiType::RustArcPtr(_) => quote!(Pointer<Void>),
        _ => todo!("FfiType::{:?}", ret_type),
    }
}

pub fn generate_ffi_dart_type(ret: Option<&FfiType>) -> dart::Tokens {
    let Some(ret_type) = ret else {
        return quote!(void)
    };
    match *ret_type {
        FfiType::UInt8 => quote!(int),
        FfiType::UInt16 => quote!(int),
        FfiType::UInt32 => quote!(int),
        FfiType::UInt64 => quote!(int),
        FfiType::Int8 => quote!(int),
        FfiType::Int16 => quote!(int),
        FfiType::Int32 => quote!(int),
        FfiType::Int64 => quote!(int),
        FfiType::Float32 | FfiType::Float64 => quote!(double),
        FfiType::RustBuffer(ref inner) => match inner {
            Some(i) => quote!($i),
            _ => quote!(RustBuffer),
        },
        FfiType::RustArcPtr(_) => quote!(Pointer<Void>),
        _ => todo!("FfiType::{:?}", ret_type),
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
        Type::Object{name, ..} => quote!($name),
        Type::Boolean => quote!(bool),
        Type::Optional( inner_type) => quote!($(generate_type(inner_type))?),
        Type::Sequence ( inner_type ) => quote!(List<$(generate_type(inner_type))>),
        Type::Enum ( name,..  ) => quote!($name),
        // Type::Record { name,..  } => quote!($name),
        _ => todo!("Type::{:?}", ty)
        // AbiType::Num(ty) => self.generate_wrapped_num_type(*ty),
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
        Type::String | Type::Optional { .. } | Type::Enum { .. } | Type::Sequence { .. }=> quote!(toRustBuffer(api, $inner)),
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
        Type::Boolean => quote!(($inner) > 0),
        Type::String => quote!(liftString(api, $inner)),
        Type::Object { name, .. } => quote!($name.lift(api, $inner)),
        Type::Enum (name, .. ) => quote!($name.lift(api, $inner)),
        Type::Optional ( inner_type ) => type_lift_optional_inner_type(inner_type, inner),
        _ => todo!("lift Type::{:?}", ty),
    }
}


fn type_lift_optional_inner_type(inner_type: &Box<Type>, inner: dart::Tokens) -> dart::Tokens {
    match **inner_type {
        Type::Int8 | Type::UInt8 => quote!(liftOptional(api, $inner, (api, v) => liftInt8OrUint8(v))),
        Type::Int16 | Type::UInt16 => quote!(liftOptional(api, $inner, (api, v) => liftInt16OrUint16(v))),
        Type::Int32 | Type::UInt32 => quote!(liftOptional(api, $inner, (api, v) => liftInt32OrUint32(v))),
        Type::Int64 | Type::UInt64 => quote!(liftOptional(api, $inner, (api, v) => liftInt64OrUint64(v))),
        Type::Float32 => quote!(liftOptional(api, $inner, (api, v) => liftFloat32(v))),
        Type::Float64 => quote!(liftOptional(api, $inner, (api, v) => liftFloat64(v))),
        Type::String => quote!(liftOptional(api, $inner, (api, v) => $(type_lift_fn(inner_type, quote!(v.sublist(5))))) ),
        _ => todo!("lift Option inner type: Type::{:?}", inner_type)
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
        | Type::Boolean => quote!((($inner) ? 1 : 0)),
        Type::String => quote!(lowerString(api, $inner)),
        Type::Object { name, .. } => quote!($name.lower(api, $inner)),
        Type::Enum ( name, .. ) => {quote!($name.lower(api, $inner))},
        Type::Optional ( inner_type ) => quote!(lowerOptional(api, $inner, (api, v) => $(type_lower_fn(inner_type, quote!(v))))),
        Type::Sequence ( inner_type ) => quote!(lowerSequence(api, value, lowerUint8, 1)), // TODO: Write try lower primitives, then check what a sequence actually looks like and replicate it
        _ => todo!("lower Type::{:?}", ty),
    }
}
