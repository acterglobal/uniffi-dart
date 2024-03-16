use uniffi;
use uniffi::{Enum, Record};


#[uniffi::export]
pub fn new_flat_one() -> FlatEnum {
    FlatEnum::One
}

#[uniffi::export]
pub fn new_flat_two() -> FlatEnum {
    FlatEnum::Two
}

#[uniffi::export]
pub fn new_flat_three() -> FlatEnum {
    FlatEnum::Three
}

#[uniffi::export]
pub fn new_flat_four() -> FlatEnum {
    FlatEnum::Four
}

#[uniffi::export]
pub fn take_flat_enum(flat: FlatEnum) -> String {
    match flat {
        FlatEnum::One => "One".to_string(),
        FlatEnum::Two => "Two".to_string(),
        FlatEnum::Three => "Three".to_string(),
        FlatEnum::Four => "Four".to_string(),
    }
}

#[uniffi::export]
pub fn new_u8_value(value: u8) -> Value {
    Value::U8 { value }
}

#[uniffi::export]
pub fn new_i8_value(value: i8) -> Value {
    Value::I8 { value }
}

#[uniffi::export]
pub fn new_u16_value(value: u16) -> Value {
    Value::U16 { value }
}

#[uniffi::export]
pub fn new_i16_value(value: i16) -> Value {
    Value::I16 { value }
}

#[uniffi::export]
pub fn new_u64_value(value: u64) -> Value {
    Value::U64 { value }
}

#[uniffi::export]
pub fn new_i64_value(value: i64) -> Value {
    Value::I64 { value }
}

#[uniffi::export]
pub fn new_u32_value(value: u32) -> Value {
    Value::U32 { value }
}

#[uniffi::export]
pub fn new_i32_value(value: i32) -> Value {
    Value::I32 { value }
}

#[uniffi::export]
pub fn new_f32_value(value: f32) -> Value {
    Value::F32 { value }
}

#[uniffi::export]
pub fn new_f64_value(value: f64) -> Value {
    Value::F64 { value }
}

#[uniffi::export]
pub fn new_string_value(value: String) -> Value {
    Value::String { value }
}

#[uniffi::export]
pub fn new_bool_value(value: bool) -> Value {
    Value::Bool { value }
}
// Holding off till refactor
#[uniffi::export]
pub fn new_public_key_value_without_argument() -> Value {
    Value::PublicKey { value: vec![3, 4, 4, 5, 4, 24434398, 4] }
}

#[uniffi::export]
pub fn new_public_key_value(value: Vec<i32>) -> Value {
    Value::PublicKey { value }
}

#[uniffi::export]
pub fn take_value(value: Value) -> String {
    match value {
        Value::String { value } => format!("{}", value),
        Value::Bool { value } => format!("{}", value),
        Value::U8 { value } => format!("{}", value),
        Value::U16 { value } => format!("{}", value),
        Value::U32 { value } => format!("{}", value),
        Value::U64 { value } => format!("{}", value),
        Value::I8 { value } => format!("{}", value),
        Value::I16 { value } => format!("{}", value),
        Value::I32 { value } => format!("{}", value),
        Value::I64 { value }  => format!("{}", value),
        Value::F32 { value }  => format!("{}", value),
        Value::F64 { value }  => format!("{}", value),
        //Value::Enum { discriminator, fields } => format!("{:?}, {:?}",discriminator, fields),
        // Value::NonHomogenousCollection { elements } => format!("{:?}", elements),
        // Value::HomogenousCollection { elements } => format!("{:?}", elements),
        // Value::Map { entries } => format!("{:?}", entries),
        Value::PublicKey { value } => format!("{:?}", value),
    }
}

#[derive(Debug, Clone, Enum)]
pub enum FlatEnum {
    One, 
    Two, 
    Three, 
    Four
}

// TODO: Add Collections (Maps, Vector, ...)
#[derive(Debug, Clone, Enum)]
pub enum Value {
    String {
        value: String,
    },
    Bool {
        value: bool,
    },
    U8 {
        value: u8,
    },
    U16 {
        value: u16,
    },
    U32 {
        value: u32,
    },
    U64 {
        value: u64,
    },

    I8 {
        value: i8,
    },
    I16 {
        value: i16,
    },
    I32 {
        value: i32,
    },
    I64 {
        value: i64,
    },
    F32 {
        value: f32,
    },
    F64 {
        value: f64
    },
    // Enum {
    //     discriminator: u8,
    //     fields: Vec<Value>,
    // },
    // HomogenousCollection {
    //     elements: Vec<Value>,
    // },
    // Map {
    //     entries: Vec<MapEntry>,
    // },

    PublicKey {
        value: Vec<i32>,
    },
}

#[derive(Clone, Debug, Record)]
pub struct MapEntry {
    pub key: Value,
    pub value: Value,
}

uniffi::include_scaffolding!("api");
