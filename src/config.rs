use std::{io::Stdin, path::PathBuf};

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

/// Config file structure
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Config {
    /// API token
    pub api_key: String,
    /// base currency
    pub base: String,
    /// endpoint url to get supported symbols (param : {api_key})
    pub symbols_endpoint_url: String,
    /// endpoint url to get conversion rates (param : {api_key}, {base})
    pub latest_endpoint_url: String,
    /// file path where supported symbols are stored
    pub symbols_file_path: String,
    // file path where conversion rates are stored
    pub conversion_rates_file_path: String,
}

#[cfg(not(tarpaulin_include))]
impl Default for Config {
    fn default() -> Self {
        // If error to find home_dir => panic
        let homedir = home::home_dir().unwrap();

        let mut symbols_file_path = PathBuf::new();
        symbols_file_path.push(&homedir);
        symbols_file_path.push(".currency-converter-cli/symbols.tsv");

        let mut conversion_rates_file_path = PathBuf::new();
        conversion_rates_file_path.push(&homedir);
        conversion_rates_file_path.push(".currency-converter-cli/conversion_rates.tsv");

        Config {
            api_key: "#INSERT_API_KEY_HERE#".to_string(),
            base: "EUR".to_string(),
            // Not sure of this one : it can cause problem in case of path with non utf-8
            // characters
            // TODO : refactor to replace String to Pathbuf
            symbols_file_path: symbols_file_path.to_string_lossy().into_owned(),
            conversion_rates_file_path: conversion_rates_file_path.to_string_lossy().into_owned(),
            latest_endpoint_url:
                "http://api.exchangeratesapi.io/v1/latest?access_key={api_key}&base={base}"
                    .to_string(),
            symbols_endpoint_url: "http://api.exchangeratesapi.io/v1/symbols?access_key={api_key}"
                .to_string(),
        }
    }
}

#[cfg(not(tarpaulin_include))]
impl Config {
    pub fn prompt_config(&self) -> Result<Config> {
        let mut res = Config::default();
        let stdin = std::io::stdin();
        let mut buffer = String::new();
        println!("Initialization of config file");

        // api key
        println!(
            "api key (required exchange rates api key)(current : {}) : ",
            self.api_key
        );
        stdin.read_line(&mut buffer)?;
        if !buffer.trim().is_empty() {
            res.api_key.clone_from(&buffer.trim().to_string());
        } else if self.api_key != "#INSERT_API_KEY_HERE#" {
            res.api_key.clone_from(&self.api_key);
        } else {
            bail!("API key must be provided !")
        }

        // base
        res.base
            .clone_from(&prompt_string(&stdin, "base currency", &self.base)?);
        // symbols file path
        res.symbols_file_path.clone_from(&prompt_string(
            &stdin,
            "currency symbols file path",
            &self.symbols_file_path,
        )?);
        // symbols endpoint
        res.symbols_endpoint_url.clone_from(&prompt_string(
            &stdin,
            "currency symbols endpoint URL",
            &self.symbols_endpoint_url,
        )?);
        // converison rates file path
        res.conversion_rates_file_path.clone_from(&prompt_string(
            &stdin,
            "conversion rates file path",
            &self.conversion_rates_file_path,
        )?);
        // conversion rates endpoint
        res.latest_endpoint_url.clone_from(&prompt_string(
            &stdin,
            "conversion rates endpoint URL",
            &self.latest_endpoint_url,
        )?);

        println!("Config initialized !");

        Ok(res)
    }
}

#[cfg(not(tarpaulin_include))]
fn prompt_string(stdin: &Stdin, text: &str, current_value: &String) -> Result<String> {
    println!("{text} (current : {current_value}) : ");
    let mut buffer = String::new();
    stdin.read_line(&mut buffer)?;
    if buffer.trim().is_empty() {
        return Ok(current_value.clone());
    }

    Ok(buffer.trim().to_string())
}
