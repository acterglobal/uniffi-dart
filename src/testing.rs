use anyhow::{bail, Result};
use camino::Utf8Path;
use std::fs::{copy, create_dir_all, File};
use std::io::Write;
use std::process::Command;
use uniffi_testing::UniFFITestHelper;

pub fn run_test(fixture: &str) -> Result<()> {
    let tmp_dir = camino_tempfile::tempdir()?;

    let script_path = Utf8Path::new(".").canonicalize_utf8()?;
    let test_helper = UniFFITestHelper::new(fixture)?;
    let out_dir = test_helper.create_out_dir(&tmp_dir, &script_path)?;

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
    let main_compile_source = test_helper.get_main_compile_source()?;

    test_helper.copy_cdylibs_to_out_dir(&out_dir)?;
    // let generated_sources =
    //     GeneratedSources::new(&test_helper.cdylib_path()?, &out_dir, &test_helper)?;
    for file in glob::glob(&format!("dart-tests/*.dart"))?.filter_map(Result::ok) {
        copy(
            &file,
            test_outdir.join(file.file_name().unwrap().to_str().unwrap()),
        )?;
    }

    // Run the test script against compiled bindings
    let mut command = Command::new("dart");
    command.current_dir(&out_dir).arg("test");
    let status = command.spawn()?.wait()?;
    if !status.success() {
        bail!("running `dart` to run test script failed ({:?})", command);
    }
    Ok(())
}
