#[cfg(feature = "build")]
mod build;
#[cfg(feature = "bindgen-tests")]
pub mod testing;
#[cfg(feature = "build")]
pub use build::generate_scaffolding;

pub mod gen;

pub use uniffi_dart_macro::*;
