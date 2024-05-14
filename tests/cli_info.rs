use assert_cmd::Command;
use currency_conversion::common::{
    conversion_rate::ConversionRate, create_or_update_file, supported_symbols::Symbols,
};
use currency_converter_cli::config::Config;
use rust_decimal_macros::dec;
use std::path::Path;

#[test]
fn cli_info() -> Result<(), Box<dyn std::error::Error>> {
    let conversion_rates = vec![ConversionRate {
        from: "EUR".to_string(),
        to: "USD".to_string(),
        rate: dec!(1.08),
    }];

    let symbols = vec![Symbols {
        code: "EUR".to_string(),
        name: "Euro".to_string(),
    }];

    let dirpath = "./temp/test/cli_info";

    std::fs::create_dir_all(dirpath).unwrap();

    let conversion_rates_path = dirpath.to_string() + "/conversion_rate.tsv";
    create_or_update_file(&conversion_rates, Path::new(&conversion_rates_path)).unwrap();

    let symbols_path = dirpath.to_string() + "/symbols.tsv";
    create_or_update_file(&symbols, Path::new(&symbols_path)).unwrap();

    let config_path = dirpath.to_string() + "/config.toml";
    let config = Config {
        conversion_rates_file_path: conversion_rates_path,
        base: "EUR".to_string(),
        symbols_file_path: symbols_path,
        ..Default::default()
    };

    confy::store_path(&config_path, config).unwrap();

    let mut cmd = Command::cargo_bin("currency-converter-cli")?;

    cmd.arg("-vvv")
        .arg("--config-path")
        .arg(config_path)
        .arg("info")
        .arg("--all")
        .assert()
        .success();

    std::fs::remove_dir_all(dirpath).unwrap();

    Ok(())
}
