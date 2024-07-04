use crate::gen;
use anyhow::{bail, Result};
use camino::{Utf8Path, Utf8PathBuf};
use std::fs::{copy, create_dir_all, File};
use std::io::Write;
use std::process::Command;
use std::thread;
use std::time::Duration;
use uniffi_testing::UniFFITestHelper;

// A source to compile for a test
#[derive(Debug)]
pub struct CompileSource {
    pub udl_path: Utf8PathBuf,
    pub config_path: Option<Utf8PathBuf>,
}

pub fn run_test(fixture: &str, udl_path: &str, config_path: Option<&str>) -> Result<()> {
    let tmp_dir = camino_tempfile::tempdir()?;

    let script_path = Utf8Path::new(".").canonicalize_utf8()?;
    let test_helper = UniFFITestHelper::new(fixture)?;
    let out_dir = test_helper.create_out_dir(&tmp_dir, &script_path)?;

    let udl_path = Utf8Path::new(".").canonicalize_utf8()?.join(udl_path);
    let config_path = if let Some(path) = config_path {
        Some(Utf8Path::new(".").canonicalize_utf8()?.join(path))
    } else {
        None
    };

    println!("{out_dir}");

    let mut pubspec = File::create(out_dir.join("pubspec.yaml"))?;
    pubspec.write(
        b"
    name: uniffi_test
    description: testing module for uniffi
    version: 1.0.0

    environment:
      sdk: '>=2.19.6 <3.0.0'
    dev_dependencies:
      test: ^1.24.3
    dependencies:
      ffi: ^2.0.1
    ",
    )?;
    pubspec.flush()?;
    let test_outdir = out_dir.join("test");
    create_dir_all(&test_outdir)?;

    test_helper.copy_cdylib_to_out_dir(&out_dir)?;
    gen::generate_dart_bindings(
        &udl_path,
        config_path.as_deref(),
        Some(&out_dir),
        Some(&test_helper.cdylib_path()?),
    )?;
    for file in glob::glob(&format!("**/*.dart"))?.filter_map(Result::ok) {
        copy(
            &file,
            out_dir.join(file.as_os_str().to_str().expect("bad filename")),
        )?;
    }

    // Run the test script against compiled bindings
    let mut command = Command::new("dart");
    command.current_dir(&out_dir).arg("test");
    let status = command.spawn()?.wait()?;
    if !status.success() {
        println!("FAILED");
        if std::env::var("CI").is_err() {
            // skip in CI environment
            thread::sleep(Duration::from_secs(120));
        }
        bail!("running `dart` to run test script failed ({:?})", command);
    }
    Ok(())
}

pub fn get_compile_sources() -> Result<Vec<CompileSource>> {
    todo!("Not implemented")
}

