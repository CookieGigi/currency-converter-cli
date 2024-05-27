use assert_cmd::Command;

#[test]
fn cli_no_sub_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("currency-conversion-cli")?;

    cmd.assert().failure();
    Ok(())
}
