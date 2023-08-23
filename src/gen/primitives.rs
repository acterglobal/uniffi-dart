use paste::paste;
use uniffi_bindgen::backend::{CodeType, Literal};
use uniffi_bindgen::interface::{Radix, Type};
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

pub fn generate_primitives_lifters() -> dart::Tokens {
    quote!{
        int? liftInt8OrUint8(Uint8List buf, [int offset = 1]) {
            return buf.isEmpty ? null : buf.buffer.asByteData().getInt8(offset);
        }

        int? liftInt16OrUint16(Uint8List buf, [int offset = 1]) {
            return buf.isEmpty ? null : buf.buffer.asByteData().getInt16(offset);
        }

        int? liftInt32OrUint32(Uint8List buf, [int offset = 1]) {
            return buf.isEmpty ? null : buf.buffer.asByteData().getInt32(offset);
        }

        int? liftInt64OrUint64(Uint8List buf, [int offset = 1]) {
            return buf.isEmpty ? null : buf.buffer.asByteData().getInt64(offset);
        }  

        double? liftFloat32(Uint8List buf, [int offset = 1]) {
            return buf.isEmpty ? null : buf.buffer.asByteData().getFloat32(offset);
        }
        
        double? liftFloat64(Uint8List buf, [int offset = 1]) {
            return buf.isEmpty ? null : buf.buffer.asByteData().getFloat64(offset);
        }

        bool? liftBoolean(Uint8List buf, [int offset = 1]) {
            return buf.isEmpty ? null : (buf.sublist(offset)[0] == 1 ? true : false);
        }
    }
}

pub fn generate_wrapper_lifters() -> dart::Tokens {
    quote! {
        class DataOffset<T> {
            final T? data;
            final int offset;
            DataOffset(this.data, this.offset);
        }
        
        // Todo!: Make this guy handle varaible strings
        DataOffset<T> liftVaraibleLength<T>(
            Uint8List buf, T? Function(Uint8List) lifter,
            [int offset = 1]) {
            final length = buf.buffer.asByteData().getInt32(offset); // the length in Uint8
            final liftedData = lifter(buf.sublist(offset + 4));
            return DataOffset(liftedData, length);
        }
    }
}

pub fn generate_primitives_lowerers() -> dart::Tokens {
    quote! {
        // TODO: Impliment lowerers for primitives        
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

        Uint8List createFixedSizedUint8ListFromFloat(double value) {
            int length = 4; // Default to 32-bit (single-precision) float
          
            if (value.isFinite) {
              if (value != value.truncateToDouble()) {
                length = 8; // 64-bit (double-precision) float
              }
            }
          
            Uint8List uint8List = Uint8List(length);
          
            ByteData byteData = ByteData.sublistView(uint8List);
            if (length == 4) {
              byteData.setFloat32(0, value, Endian.little);
            } else {
              byteData.setFloat64(0, value, Endian.little);
            }
          
            return uint8List;
          }
    }
}

pub fn generate_wrapper_lowerers() -> dart::Tokens {
    quote! {
        Uint8List lowerVaraibleLength<T>(Api api, T input, Uint8List Function(Api, T) lowerer) {
            final lowered = lowerer(api, input);
            final length = createUint8ListFromInt(lowered.length);
            Uint8List res = Uint8List(lowered.length + length.length);
            res.setAll(0, length);
            res.setAll(length.length, lowered);
            return res;
        }
    }
}

macro_rules! impl_code_type_for_primitive {
    ($T:ty, $class_name:literal) => {
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
            }
        }
    };
}

impl_code_type_for_primitive!(BooleanCodeType, "bool");
impl_code_type_for_primitive!(StringCodeType, "String");
impl_code_type_for_primitive!(BytesCodeType, "Uint8List");
impl_code_type_for_primitive!(Int8CodeType, "int");
impl_code_type_for_primitive!(Int16CodeType, "int");
impl_code_type_for_primitive!(Int32CodeType, "int");
impl_code_type_for_primitive!(Int64CodeType, "int");
impl_code_type_for_primitive!(UInt8CodeType, "int");
impl_code_type_for_primitive!(UInt16CodeType, "int");
impl_code_type_for_primitive!(UInt32CodeType, "int");
impl_code_type_for_primitive!(UInt64CodeType, "int");
impl_code_type_for_primitive!(Float32CodeType, "double");
impl_code_type_for_primitive!(Float64CodeType, "double");
