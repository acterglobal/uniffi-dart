use anyhow::Result;

#[test]
fn streams_ext() -> Result<()> {
    uniffi_dart::testing::run_test("streams_ext", "src/api.udl", None)
}
