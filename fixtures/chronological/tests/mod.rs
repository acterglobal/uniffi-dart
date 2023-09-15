use anyhow::Result;

#[test]
fn chronological() -> Result<()> {
    uniffi_dart::testing::run_test("chronological", "src/api.udl", None)
}
