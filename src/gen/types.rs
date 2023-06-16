use genco::prelude::*;
use uniffi_bindgen::interface::{FfiType, Type};

pub fn generate_ffi_type(ret: Option<&FfiType>) -> dart::Tokens {
    let Some(ret_type) = ret else {
        return quote!(Void)
    };
    match *ret_type {
        FfiType::UInt32 => quote!(ffi.Uint32),
        _ => todo!(),
    }
}

pub fn generate_ffi_dart_type(ret: Option<&FfiType>) -> dart::Tokens {
    let Some(ret_type) = ret else {
        return quote!(void)
    };
    match *ret_type {
        FfiType::UInt32 => quote!(int),
        _ => todo!(),
    }
}

pub fn generate_type(ty: &Type) -> dart::Tokens {
    match ty {
        Type::UInt32 => quote!(int),
        _ => todo!()
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

pub fn generate_type_lift_fn(ty: &Type) -> Option<dart::Tokens> {
    match ty {
        Type::UInt32 => None,
        _ => todo!(),
    }
}
