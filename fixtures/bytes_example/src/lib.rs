use uniffi;

#[uniffi::export]
pub fn make_byte_array() -> Vec<u8> {
    vec![1, 2, 3, 4, 5]
}

#[uniffi::export]
pub fn make_large_byte_array(size: u32) -> Vec<u8> {
    (0..size as u8).collect()
}

#[uniffi::export]
pub fn append_byte(mut array: Vec<u8>, value: u8) -> Vec<u8> {
    array.push(value);
    array
}

#[uniffi::export]
pub fn remove_byte(mut array: Vec<u8>, index: u32) -> Vec<u8> {
    array.remove(index as usize);
    array
}

#[uniffi::export]
pub fn clear_byte_array() -> Vec<u8> {
    vec![]
}

uniffi::include_scaffolding!("api");
