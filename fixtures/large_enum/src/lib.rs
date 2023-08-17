use uniffi;
use uniffi::{Enum, Record};

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
pub fn new_string_value(value: String) -> Value {
    Value::String { value }
}

#[uniffi::export]
pub fn new_bool_value(value: bool) -> Value {
    Value::Bool { value }
}

// #[uniffi::export]
// pub fn new_i64_value(value: i64) -> Value {
//     Value::I64 { value }
// }

// #[uniffi::export]
// pub fn new_u64_value(value: u64) -> Value {
//     Value::U64 { value }
// }

// #[uniffi::export]
// pub fn new_i64_value(value: i64) -> Value {
//     Value::I64 { value }
// }
// #[uniffi::export]
// pub fn new_u64_value(value: u64) -> Value {
//     Value::U64 { value }
// }

// #[uniffi::export]
// pub fn new_i64_value(value: i64) -> Value {
//     Value::I64 { value }
// }

// #[uniffi::export]
// pub fn take_value(value: Value)  {
//     match value {
//         Value::String { value } => todo!(),
//         Value::Bool { value } => todo!(),
//         Value::U8 { value } => todo!(),
//         Value::U16 { value } => todo!(),
//         Value::U32 { value } => todo!(),
//         Value::U64 { value } => todo!(),
//         Value::I8 { value } => todo!(),
//         Value::I16 { value } => todo!(),
//         Value::I32 { value } => todo!(),
//         Value::I64 { value } => todo!(),
//        // Value::Enum { discriminator, fields } => todo!(),
//     }
// }

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
    // Enum {
    //     discriminator: u8,
    //     fields: Vec<Value>,
    // },
    // NonHomogenousCollection {
    //     elements: Vec<Value>,
    // },
    // HomogenousCollection {
    //     elements: Vec<Value>,
    // },
    // Map {
    //     entries: Vec<MapEntry>,
    // },
    // PublicKey {
    //     value: Vec<u8>,
    // },
    // Signature {
    //     value: Vec<u8>,
    // },
}

#[derive(Clone, Debug, Record)]
pub struct MapEntry {
    pub key: Value,
    pub value: Value,
}

include!(concat!(env!("OUT_DIR"), "/api.uniffi.rs"));
