use anyhow::Result;

#[test]
fn simple_arithmetic() -> Result<()> {
    uniffi_dart::testing::run_test("simple_arithmetic", "src/api.udl", None)
}
