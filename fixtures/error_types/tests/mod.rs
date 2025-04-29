use anyhow::Result;

#[test]
fn error_types() -> Result<()> {
    uniffi_dart::testing::run_test("error_types", "src/api.udl", None)
}
