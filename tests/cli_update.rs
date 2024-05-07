use std::path::Path;

use assert_cmd::Command;
use currency_converter_cli::config::Config;
use httpmock::{Method::GET, MockServer};
use serde_json::json;

#[test]
fn cli_update() -> Result<(), Box<dyn std::error::Error>> {
    // server mocking
    // param
    let api_key = "123";
    let base = "EUR";
    let server_response_conversion_rate = json!({
        "success": true,
        "timestamp": 1519296206,
        "base": "EUR",
        "date": "2021-03-17",
        "rates": {
            "GBP": 0.72007,
            "JPY": 107.346001,
            "USD": 0.813399,
        }
    });
    let server_response_symbols = json!(
    {
      "success": true,
      "symbols": {
        "AED": "United Arab Emirates Dirham",
        "AFN": "Afghan Afghani",
        "ALL": "Albanian Lek",
        "AMD": "Armenian Dram",
        }
    }
    );

    let server = MockServer::start();

    // mock latest endpoint
    let mock_conversion_rate = server.mock(|when, then| {
        when.method(GET)
            .path("/latest")
            .query_param("access_key", api_key)
            .query_param("base", base);
        then.status(200)
            .header("content-type", "application/json")
            .json_body(server_response_conversion_rate);
    });

    let mock_symbols = server.mock(|when, then| {
        when.method(GET)
            .path("/symbols")
            .query_param("access_key", api_key);
        then.status(200)
            .header("content-type", "application/json")
            .json_body(server_response_symbols);
    });

    // modify config
    let config_path = "./tests/test-config.toml";
    let mut config: Config = confy::load_path(config_path).unwrap();

    config.latest_endpoint_url = server.url("/latest") + "?access_key={api_key}&base={base}";
    config.symbols_endpoint_url = server.url("/symbols") + "?access_key={api_key}";
    config.base = base.to_string();
    config.api_key = api_key.to_string();
    config.conversion_rates_file_path = "./tests/conversion_rates.tsv".to_string();
    config.symbols_file_path = "./tests/symbols.tsv".to_string();

    confy::store_path("./tests/test-config.toml", &config).unwrap();

    // exec command
    let mut cmd = Command::cargo_bin("currency-converter-cli")?;

    // command working
    cmd.arg("-vvv")
        .arg("--config-path")
        .arg("./tests/test-config.toml")
        .arg("update")
        .assert()
        .success();

    // server assert
    mock_symbols.assert();
    mock_conversion_rate.assert();

    // file is created
    assert!(Path::new(&config.symbols_file_path).exists());
    assert!(Path::new(&config.conversion_rates_file_path).exists());

    // check file content
    let mut csv_rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(&config.conversion_rates_file_path)?;

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
