use genco::prelude::*;
use uniffi_bindgen::interface::{FfiType, Type};

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
        Type::Optional{ inner_type} => quote!($(generate_type(inner_type))?),
        Type::Sequence { inner_type } => quote!(List<$(generate_type(inner_type))>),
        Type::Enum { name,..  } => quote!($name),
        Type::Record { name,..  } => quote!($name),
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
        Type::String | Type::Optional { .. } | Type::Enum { .. } => quote!(toRustBuffer(api, $inner)),
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
        Type::Enum { name, .. } => quote!($name.lift(api, $inner)),
        Type::Optional { inner_type } => type_lift_optional_inner_type(inner_type, inner),
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
        Type::Enum { name, .. } => {quote!($name.lower(api, $inner))},
        Type::Optional { inner_type } => quote!(lowerOptional(api, $inner, (api, v) => $(type_lower_fn(inner_type, quote!(v))))),
        _ => todo!("lower Type::{:?}", ty),
    }
}
