use anyhow::Result;

#[test]
fn fixture_callbacks() -> Result<()> {
    uniffi_dart::testing::run_test("fixture_callbacks", "src/api.udl", None)
}
