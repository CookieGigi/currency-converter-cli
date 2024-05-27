use std::path::Path;

use assert_cmd::Command;
use currency_conversion::common::{conversion_rate::ConversionRate, create_or_update_file};
use currency_conversion_cli::config::Config;
use predicates::prelude::predicate;
use rust_decimal_macros::dec;

#[test]
fn cli_convert() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![ConversionRate {
        from: "EUR".to_string(),
        to: "USD".to_string(),
        rate: dec!(1.08),
    }];

    let dirpath = "./temp/test/cli_convert";

    std::fs::create_dir_all(dirpath).unwrap();

    let path = dirpath.to_string() + "/conversion_rate.tsv";
    create_or_update_file(&data, Path::new(&path)).unwrap();

    let config_path = dirpath.to_string() + "/config.toml";
    let config = Config {
        conversion_rates_file_path: path,
        base: "EUR".to_string(),
        api_key: "test".to_string(),
        ..Default::default()
    };

    confy::store_path(&config_path, config).unwrap();

    let mut cmd = Command::cargo_bin("currency-conversion-cli")?;

    cmd.arg("-vvv")
        .arg("--config-path")
        .arg(config_path)
        .arg("convert")
        .arg("--from")
        .arg("EUR")
        .arg("--to")
        .arg("USD")
        .arg("10")
        .assert()
        .success()
        .stdout(predicate::str::contains("10.8"));

    std::fs::remove_dir_all(dirpath).unwrap();

    Ok(())
}
