use uniffi;

#[uniffi::export]
pub fn add(left: u32, right: u32) -> Option<u32> {
    Some(left + right)
}

#[uniffi::export]
pub fn multiply(left: u32, right: u32) -> Option<u32> {
    Some(left * right)
}

#[uniffi::export]
pub fn devide(left: u32, right: u32) -> Option<u32> {
   Some(left / right)
}

#[uniffi::export]
pub fn devide_checked(left: u32, right: u32) -> Option<u32> {
    left.checked_div(right)
}

#[uniffi::export]
pub fn add_u8(left: u8, right: u8) -> Option<u8> {
    Some(left + right)
}

#[uniffi::export]
pub fn add_u16(left: u16, right: u16) -> Option<u16> {
    Some(left + right)
}

#[uniffi::export]
pub fn add_u64(left: u64, right: u64) -> Option<u64> {
    Some(left + right)
}

#[uniffi::export]
pub fn add_i8(left: i8, right: i8) -> Option<i8> {
    Some(left + right)
}

#[uniffi::export]
pub fn add_i16(left: i16, right: i16) -> Option<i16> {
    Some(left + right)
}

#[uniffi::export]
pub fn add_i32(left: i32, right: i32) -> Option<i32> {
    Some(left + right)
}

#[uniffi::export]
pub fn add_i64(left: i64, right: i64) -> Option<i64> {
   Some( left + right)
}

#[uniffi::export]
pub fn add_f32(left: f32, right: f32) -> Option<f32> {
   Some(left + right)
}

#[uniffi::export]
pub fn add_f64(left: f64, right: f64) -> Option<f64> {
   Some(left + right)
}

include!(concat!(env!("OUT_DIR"), "/api.uniffi.rs"));
