use paste::paste;
use uniffi_bindgen::backend::{CodeOracle, CodeType, Literal};
use uniffi_bindgen::interface::{types::Type, Radix};

fn render_literal(_oracle: &dyn CodeOracle, literal: &Literal) -> String {
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
    ($T:ty, $class_name:literal) => {
        paste! {
            #[derive(Debug)]
            pub struct $T;

            impl CodeType for $T  {
                fn type_label(&self, _oracle: &dyn CodeOracle) -> String {
                    $class_name.into()
                }

                fn literal(&self, oracle: &dyn CodeOracle, literal: &Literal) -> String {
                    render_literal(oracle, &literal)
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
