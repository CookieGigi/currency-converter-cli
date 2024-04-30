use std::path::Path;

use assert_cmd::Command;

#[test]
fn cli_no_sub_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("currency-converter-cli")?;

    cmd.assert().failure();
    Ok(())
}

#[test]
fn cli_update() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("currency-converter-cli")?;

    // command working
    cmd.arg("update").assert().success();

    // file is created
    assert!(Path::new("./currency-conversion-rates.tsv").exists());

    Ok(())
}
