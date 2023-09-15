use anyhow::Result;

#[test]
fn ext_types() -> Result<()> {
    uniffi_dart::testing::run_test("ext_types", "src/api.udl", None)
}
