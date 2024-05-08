use uniffi;

#[uniffi::export]
pub fn add_duration_seconds(left: u64, right: u64) -> u64 {
    (left as i64 + right as i64) as u64
}

#[uniffi::export]
pub fn add_duration_milliseconds(left: u64, right: u64) -> u64 {
    left + right
}

#[uniffi::export]
pub fn add_duration_microseconds(left: u64, right: u64) -> u64 {
    left + right
}

#[uniffi::export]
pub fn get_back_duration_seconds(value: u64) -> u64 {
    value
}

#[uniffi::export]
pub fn get_back_duration_milliseconds(value: u64) -> u64 {
    value
}

#[uniffi::export]
pub fn get_back_duration_microseconds(value: u64) -> u64 {
    value
}

uniffi::include_scaffolding!("api");
