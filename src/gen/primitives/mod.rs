#[macro_use]
mod macros;
mod boolean;
mod string;
mod duration;

use crate::gen::render::{Renderable, TypeHelperRenderer};
use genco::prelude::*;
use paste::paste;
use uniffi_bindgen::backend::Literal;
use uniffi_bindgen::interface::{Radix, Type};

pub use boolean::BooleanCodeType;
pub use string::StringCodeType;
pub use duration::DurationCodeType;

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
            | Type::Float64
            | Type::Duration => num_str,
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

// TODO: implement BytesCodeType
impl_renderable_for_primitive!(BytesCodeType, "Uint8List", "Uint8List", 1);
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

        List<T> liftSequence<T>( Uint8List buf, Function(Uint8List, [int offset]) lifter, [int element_byte_size = 1,int offset = 0]) {
            List<T> res = [];
            buf = buf.sublist(offset);
            final length = buf.buffer.asByteData().getInt32(0);
            buf = buf.sublist(4);

            final element_byte_size = (buf.length ~/ length);
            offset = 0;

            for (var i = 0; i < length; i++) {
                offset = element_byte_size * i; // Update the offset for the next loop
                final item = lifter(buf, offset);
                res.add(item);
            }

            return res;
        }
    }
}

pub fn generate_wrapper_lowerers() -> dart::Tokens {
    quote! {
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

        Uint8List lowerVaraibleLength<T>( T input, Uint8List Function(Api, T) lowerer) {
            final lowered = lowerer(api, input);
            final length = createUint8ListFromInt(lowered.length);
            Uint8List res = Uint8List(lowered.length + length.length);
            res.setAll(0, length);
            res.setAll(length.length, lowered);
            return res;
        }


        Uint8List lowerSequence<T, V>( List<T> input, Uint8List Function(Api, V) lowerer, int element_byte_size) {
          int capacity = input.length * element_byte_size;
          Uint8List items = Uint8List(capacity + 4); // Four bytes for the length
          int offset = 0;

          // Set the length of the vec
          items.setAll(offset, createUint8ListFromInt(capacity));
          offset += 4;

          for (var i = 0; i < input.length; i++) {
            items.setRange(
                offset, offset + element_byte_size, lowerer(api, input[i] as V));
            offset += element_byte_size;
          }

          print("Items from sequence");
          print(items);
          return items;
        }
    }
}

