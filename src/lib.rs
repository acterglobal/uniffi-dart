#[cfg(feature = "bindgen-tests")]
pub mod testing;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
