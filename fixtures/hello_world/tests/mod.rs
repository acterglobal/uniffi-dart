use anyhow::Result;

#[test]
fn hello_world() -> Result<()> {
    uniffi_dart::testing::run_test("hello_world", "src/api.udl", None)
}
