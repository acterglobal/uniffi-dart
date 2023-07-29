use anyhow::Result;

#[test]
fn large_enum() -> Result<()> {
    uniffi_dart::testing::run_test("large_enum", "src/api.udl", None)
}
