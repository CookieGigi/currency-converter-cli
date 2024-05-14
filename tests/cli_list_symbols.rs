use std::path::Path;

use assert_cmd::Command;
use currency_conversion::common::{create_or_update_file, supported_symbols::Symbols};
use currency_converter_cli::config::Config;
use predicates::prelude::predicate;

#[test]
fn cli_get_symbols() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![
        Symbols {
            code: "EUR".to_string(),
            name: "Euro".to_string(),
        },
        Symbols {
            code: "USD".to_string(),
            name: "United state dollard".to_string(),
        },
    ];

    let dirpath = "./temp/test/cli_get_symbols";

    std::fs::create_dir_all(dirpath).unwrap();

    let path = dirpath.to_string() + "/symbols.tsv";
    create_or_update_file(&data, Path::new(&path)).unwrap();

    let config_path = dirpath.to_string() + "/config.toml";
    let config = Config {
        symbols_file_path: path,
        ..Default::default()
    };

    confy::store_path(&config_path, config).unwrap();

    let mut cmd = Command::cargo_bin("currency-converter-cli")?;

    cmd.arg("-vvv")
        .arg("--config-path")
        .arg(config_path)
        .arg("list")
        .arg("symbols")
        .assert()
        .success()
        .stdout(predicate::str::contains("EUR : Euro"));

    std::fs::remove_dir_all(dirpath).unwrap();

    Ok(())
}
