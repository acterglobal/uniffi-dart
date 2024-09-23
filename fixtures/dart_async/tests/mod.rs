use anyhow::Result;

#[test]
fn dart_async() -> Result<()> {
    uniffi_dart::testing::run_test("dart_async", "src/api.udl", None)
}
