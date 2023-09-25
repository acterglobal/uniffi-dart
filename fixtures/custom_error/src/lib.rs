use anyhow::{bail, Error as AnyhowError};
use uniffi;

#[derive(uniffi::Error)]
pub struct CustomError;

#[uniffi::export]
pub fn anyhow_error() -> Result<(), AnyhowError> {
    bail!("this is an anyhow error")
}

#[uniffi::export]
pub fn internal_panic() {
    panic!("This is a panic")
}

#[uniffi::export]
pub fn custom_error() -> Result<(), CustomError> {
    Err(CustomError)
}

mod uniffi_types {
    pub use crate::CustomError;
    pub use anyhow::Error as AnyhowError;
}

uniffi::include_scaffolding!("api");