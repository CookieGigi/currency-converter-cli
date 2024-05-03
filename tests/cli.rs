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
    let path = "./currency-conversion-rates.tsv";
    assert!(Path::new(path).exists());

    // check file content
    let mut csv_rdr = csv::ReaderBuilder::new().delimiter(b'\t').from_path(path)?;

    // header
    {
        let headers = csv_rdr.headers()?;
        assert_eq!(headers, vec!["from", "to", "rate"]);
    }

    // content
    let first_row = csv_rdr.records().next();
    assert!(first_row.is_some());
    assert!(first_row.unwrap().is_ok());

    Ok(())
}
