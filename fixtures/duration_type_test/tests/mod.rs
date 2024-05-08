use anyhow::Result;

#[test]
fn duration_type_test() -> Result<()> {
    uniffi_dart::testing::run_test("duration_type_test", "src/api.udl", None)
}
