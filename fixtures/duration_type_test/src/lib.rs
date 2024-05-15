use core::time::Duration;
use uniffi;

#[uniffi::export]
pub fn add_duration(seconds: i64, nanos: i32) -> Duration {
    Duration::new(seconds as u64, nanos as u32)
}

#[uniffi::export]
pub fn get_seconds(duration: Duration) -> i64 {
    duration.as_secs() as i64
}

#[uniffi::export]
pub fn get_nanos(duration: Duration) -> i32 {
    duration.as_nanos() as i32 % 1_000_000_000
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_add_duration() {
//         assert_eq!(get_seconds(add_duration(1, 1)), 1);
//         assert_eq!(get_nanos(add_duration(1, 1)), 1);
//     }
// }
uniffi::include_scaffolding!("api");
