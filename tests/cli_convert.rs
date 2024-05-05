use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn cli_convert() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("currency-converter-cli")?;

    cmd.arg("-vvv")
        .arg("convert")
        .arg("--from")
        .arg("EUR")
        .arg("--to")
        .arg("USD")
        .arg("10")
        .assert()
        .success()
        .stdout(predicate::str::contains("10.8"));

    Ok(())
}
