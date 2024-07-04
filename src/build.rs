use anyhow::{Context, Result};
use camino::Utf8Path;
use std::env;

pub fn generate_scaffolding(udl_file: &Utf8Path) -> Result<()> {
    uniffi_build::generate_scaffolding(udl_file)?;
    let out_dir = env::var("OUT_DIR").context("$OUT_DIR missing?!")?;
    uniffi_bindgen::generate_external_bindings(
        &crate::gen::DartBindingGenerator {},
        udl_file,
        None::<&Utf8Path>,
        Some(out_dir),
        None::<&Utf8Path>,
        None,
        true,
    )?;
    Ok(())
}

