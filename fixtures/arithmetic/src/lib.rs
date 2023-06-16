use uniffi;

#[uniffi::export]
pub fn add(left: u32, right: u32) -> u32 {
    left + right
}

#[uniffi::export]
pub fn multiply(left: u32, right: u32) -> u32 {
    left * right
}

include!(concat!(env!("OUT_DIR"), "/api.uniffi.rs"));
