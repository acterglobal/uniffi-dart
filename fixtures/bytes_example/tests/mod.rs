use anyhow::Result;

#[test]
fn bytes_example() -> Result<()> {
    uniffi_dart::testing::run_test("bytes_example", "src/api.udl", None)
}
