use uniffi;

#[uniffi::export]
pub fn add(left: u32, right: u32) -> u32 {
    left + right
}

#[uniffi::export]
pub fn multiply(left: u32, right: u32) -> u32 {
    left * right
}

#[uniffi::export]
pub fn devide(left: u32, right: u32) -> u32 {
    left / right
}

#[uniffi::export]
pub fn devide_checked(left: u32, right: u32) -> Option<u32> {
    left.checked_div(right)
}

#[uniffi::export]
pub fn add_u8(left: u8, right: u8) -> u8 {
    left + right
}

#[uniffi::export]
pub fn add_u16(left: u16, right: u16) -> u16 {
    left + right
}

#[uniffi::export]
pub fn add_u64(left: u64, right: u64) -> u64 {
    left + right
}

#[uniffi::export]
pub fn add_i8(left: i8, right: i8) -> i8 {
    left + right
}

#[uniffi::export]
pub fn add_i16(left: i16, right: i16) -> i16 {
    left + right
}

#[uniffi::export]
pub fn add_i32(left: i32, right: i32) -> i32 {
    left + right
}

#[uniffi::export]
pub fn add_i64(left: i64, right: i64) -> i64 {
    left + right
}

include!(concat!(env!("OUT_DIR"), "/api.uniffi.rs"));
