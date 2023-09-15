use anyhow::Result;

#[test]
fn custom_types() -> Result<()> {
    uniffi_dart::testing::run_test("custom_types", "src/api.udl", None)
}
