use paste::paste;
use uniffi_bindgen::backend::{CodeType, Literal};
use uniffi_bindgen::interface::{Radix, Type};
use crate::gen::render::{Renderable, TypeHelperRenderer};
use crate::gen::oracle::DartCodeOracle;
use genco::prelude::*;

fn render_literal(literal: &Literal) -> String {
    fn typed_number(type_: &Type, num_str: String) -> String {
        match type_ {
            Type::Int8
            | Type::UInt8
            | Type::Int16
            | Type::UInt16
            | Type::Int32
            | Type::UInt32
            | Type::UInt64
            | Type::Float32
            | Type::Float64 => num_str,
            _ => panic!("Unexpected literal: {} is not a number", num_str),
        }
    }

    match literal {
        Literal::Boolean(v) => format!("{}", v),
        Literal::String(s) => format!("\"{}\"", s),
        Literal::Int(i, radix, type_) => typed_number(
            type_,
            match radix {
                Radix::Octal => format!("{:#x}", i),
                Radix::Decimal => format!("{}", i),
                Radix::Hexadecimal => format!("{:#x}", i),
            },
        ),
        Literal::UInt(i, radix, type_) => typed_number(
            type_,
            match radix {
                Radix::Octal => format!("{:#x}", i),
                Radix::Decimal => format!("{}", i),
                Radix::Hexadecimal => format!("{:#x}", i),
            },
        ),
        Literal::Float(string, type_) => typed_number(type_, string.clone()),

        _ => unreachable!("Literal"),
    }
}

macro_rules! impl_code_type_for_primitive {
    ($T:ty, $class_name:literal, $canonical_name:literal) => {
        paste! {
            #[derive(Debug)]
            pub struct $T;

            impl CodeType for $T  {
                fn type_label(&self,) -> String {
                    $class_name.into()
                }

                fn literal(&self, literal: &Literal) -> String {
                    render_literal(&literal)
                }

                fn canonical_name(&self,) -> String {
                    $canonical_name.into()
                }

                fn ffi_converter_name(&self) -> String {
                    format!("{}FfiConverter", self.canonical_name())
                }
            
                fn lower(&self) -> String {
                    format!("{}.lower", self.ffi_converter_name())
                }
            
                fn write(&self) -> String {
                    format!("{}.write", self.ffi_converter_name())
                }
            
                fn lift(&self) -> String {
                    format!("{}.lift", self.ffi_converter_name())
                }
            
                fn read(&self) -> String {
                    format!("{}.read", self.ffi_converter_name())
                }
            }
        }
    };
}

macro_rules! impl_renderable_for_primitive {
    ($T:ty, $class_name:literal, $canonical_name:literal, $allocation_size:literal) => {
        impl Renderable for $T {
            fn render_type(&self, _ty: &Type) -> dart::Tokens {
                quote!($class_name)
            }

            fn render_type_helpers(&self, _ty: &Type, type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
                if !(type_helper.include_once_check($canonical_name)) {
                    return quote!("") // Return an empty string to avoid code duplication
                }
                // This method can be expanded to generate type helper methods if needed.
                let endian = (if $canonical_name.contains("Float") { "Endian.little" } else { "Endian.big" });
                let final_uintlist = (if $canonical_name.contains("Float") { 
                    String::from($canonical_name) + "List.fromList(buf.reversed.toList())" 
                } else {
                    String::from($canonical_name) + "List.fromList(buf.toList())" 
                });

                let cl_name = String::from($canonical_name) + "FfiConverter";

                quote! {
                    class $cl_name extends FfiConverter<$canonical_name, RustBuffer> {
                        @override
                        int lift(Api api, RustBuffer buf) {
                            final uint_list = buf.toIntList();
                            return uint_list.buffer.asByteData().get$canonical_name(1);
                        }
                      
                        @override
                        RustBuffer lower(Api api, int value) {
                            final buf = Uint8List(this.allocationSize());
                            final byteData = ByteData.sublistView(buf);
                            byteData.set$canonical_name(0, value, $endian);
                            return toRustBuffer($final_uintlist);
                        }
                      
                        @override
                        int read(ByteBuffer buf) {
                            // So here's the deal, we have two choices, could use Uint8List or ByteBuffer, leaving this for later
                            // considerations, after research on performance implications
                          throw UnimplementedError("Should probably impliment read now");
                        }
                      
                        @override
                        int allocationSize(int value) {
                          return $allocation_size; 
                        }
                      
                        @override
                        void write(int value, ByteBuffer buf) {
                            throw UnimplementedError("Should probably impliment writes now");
                        }
                    }                      
                }
            }
        }
    };

    (BooleanCodeType, $class_name:literal, $canonical_name:literal, $allocation_size:literal) => {
        impl Renderable for $T {
            fn render_type(&self, _ty: &Type) -> dart::Tokens {
                quote!($class_name)
            }

            fn render_type_helpers(&self, _ty: &Type, type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
                if !(type_helper.include_once_check($canonical_name)) {
                    return quote!("") // Return an empty string to avoid code duplication
                }
                // This method can be expanded to generate type helper methods if needed.
                quote! {
                    class BooleanFfiConverter extends FfiConverter<$canonical_name, RustBuffer> {
                        @override
                        bool lift(Api api, RustBuffer buf) {
                            final uint_list = buf.toIntList();
                            return uint_list.buffer.sublist(offset)[0] == 1;
                        }
                      
                        @override
                        RustBuffer lower(Api api, bool value) {
                            final uint_list = Uint8List.fromList([value ? 1 : 0]);
                            return toRustBuffer(uint_list);
                        }
                      
                        @override
                        int read(ByteBuffer buf) {
                            // So here's the deal, we have two choices, could use Uint8List or ByteBuffer, leaving this for later
                            // performance reasons
                          throw UnimplementedError("Should probably impliment read now");
                        }
                      
                        @override
                        int allocationSize(int value) {
                          return $allocation_size; // 1 = 8bits
                        }
                      
                        @override
                        void write(int value, ByteBuffer buf) {
                            throw UnimplementedError("Should probably impliment read now");
                        }
                    }                 
                }
            }
        }
    };
    
    (StringCodeType, $class_name:literal, $canonical_name:literal, $allocation_size:literal) => {
        impl Renderable for $T {
            fn render_type(&self, _ty: &Type) -> dart::Tokens {
                quote!($class_name)
            }

            fn render_type_helpers(&self, _ty: &Type, type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
                // This method can be expanded to generate type helper methods if needed.
                quote! {
                    if !(type_helper.include_once_check($canonical_name)) {
                        return quote!("") // Return an empty string to avoid code duplication
                    }
                    class StringFfiConverter extends FfiConverter<$canonical_name, RustBuffer> {
                        @override
                        int lift(Api api, RustBuffer buf) {
                            final uint_list = buf.toIntList();
                            return utf8.decoder.convert(uint_list);
                        }
                      
                        @override
                        RustBuffer lower(Api api, String value) {
                            // FIXME: this is too many memcopies!
                            return Utf8Encoder().convert(input);
                        }
                      
                        @override
                        int read(ByteBuffer buf) {
                            // So here's the deal, we have two choices, could use Uint8List or ByteBuffer, leaving this for later
                            // performance reasons
                          throw UnimplementedError("Should probably impliment read now");
                        }
                      
                        @override
                        int allocationSize(int value) {
                          return $allocation_size; // 1 = 8bits
                        }
                      
                        @override
                        void write(int value, ByteBuffer buf) {
                            throw UnimplementedError("Should probably impliment read now");
                        }
                    }                 
                }
            }
        }
    };

    (BytesCodeType, $class_name:literal, $canonical_name:literal, $allocation_size:literal) => {
        impl Renderable for $T {
            fn render_type(&self, _ty: &Type) -> dart::Tokens {
                quote!($class_name)
            }

            fn render_type_helpers(&self, _ty: &Type, type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
                if !(type_helper.include_once_check($canonical_name)) {
                    return quote!("") // Return an empty string to avoid code duplication
                }
                // TODO: Impliment bytes ffi methods
                quote! {
                    class BytesFfiConverter extends FfiConverter<$canonical_name, RustBuffer> {
                        @override
                        int lift(Api api, RustBuffer buf) {
                            // final uint_list = buf.toIntList();
                            // return uint_list.buffer.asByteData().get$canonical_name(1);
                        }
                      
                        @override
                        RustBuffer lower(Api api, int value) {
                            // final uint_list = Uint8List.fromList([value ? 1 : 0]);
                            // final byteData = ByteData.sublistView(buf);
                            // byteData.setInt16(0, value, Endian.little);
                            // return buf;
                        }
                      
                        @override
                        int read(ByteBuffer buf) {
                        //     // So here's the deal, we have two choices, could use Uint8List or ByteBuffer, leaving this for later
                        //     // performance reasons
                        //   throw UnimplementedError("Should probably impliment read now");
                        }
                      
                        @override
                        int allocationSize(int value) {
                        //   return $allocation_size; // 1 = 8bits
                        }
                      
                        @override
                        void write(int value, ByteBuffer buf) {
                            // throw UnimplementedError("Should probably impliment read now");
                        }
                    }                 
                }
            }
        }
    }
}

// TODO: Impliment separate for string

impl_code_type_for_primitive!(BooleanCodeType, "bool", "Bool");
impl_code_type_for_primitive!(StringCodeType, "String", "String");
impl_code_type_for_primitive!(BytesCodeType, "Uint8List", "Uint8List");
impl_code_type_for_primitive!(Int8CodeType, "int", "Int8");
impl_code_type_for_primitive!(Int16CodeType, "int", "Int16");
impl_code_type_for_primitive!(Int32CodeType, "int", "Int32");
impl_code_type_for_primitive!(Int64CodeType, "int", "Int64");
impl_code_type_for_primitive!(UInt8CodeType, "int", "UInt8");
impl_code_type_for_primitive!(UInt16CodeType, "int", "UInt16");
impl_code_type_for_primitive!(UInt32CodeType, "int", "UInt32");
impl_code_type_for_primitive!(UInt64CodeType, "int", "UInt64");
impl_code_type_for_primitive!(Float32CodeType, "double", "Double32");
impl_code_type_for_primitive!(Float64CodeType, "double", "Double64");

impl_renderable_for_primitive!(BooleanCodeType, "bool", "Bool", 1);
impl_renderable_for_primitive!(StringCodeType, "String", "String", 1);
// TODO: Impliment BytesCodeType
// impl_renderable_for_primitive!(BytesCodeType, "Uint8List", "Uint8List", 1);
impl_renderable_for_primitive!(Int8CodeType, "int", "Int8", 1);
impl_renderable_for_primitive!(Int16CodeType, "int", "Int16", 2);
impl_renderable_for_primitive!(Int32CodeType, "int", "Int32", 4);
impl_renderable_for_primitive!(Int64CodeType, "int", "Int64", 8);
impl_renderable_for_primitive!(UInt8CodeType, "int", "UInt8", 1);
impl_renderable_for_primitive!(UInt16CodeType, "int", "UInt16", 2);
impl_renderable_for_primitive!(UInt32CodeType, "int", "UInt32", 4);
impl_renderable_for_primitive!(UInt64CodeType, "int", "UInt64", 8);
impl_renderable_for_primitive!(Float32CodeType, "double", "Double32", 4);
impl_renderable_for_primitive!(Float64CodeType, "double", "Double64", 8);
