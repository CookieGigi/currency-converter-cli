use std::{collections::HashMap, path::Path};

use anyhow::anyhow;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::*;

use self::config::Config;

use super::common::{self, conversion_rate::ConversionRate, supported_symbols::Symbols};

pub fn run_update(config: Config) -> Result<()> {
    update_symbols(&config)?;

    update_conversion_rates(&config)?;
    Ok(())
}

fn update_conversion_rates(config: &Config) -> Result<()> {
    let data = get_conversion_rates(&config.api_key, &config.base)?;

    let path = Path::new("./currency-conversion-rates.tsv");

    create_or_update_file(&data, path)?;

    Ok(())
}

fn update_symbols(config: &Config) -> Result<()> {
    // symbols
    let symbols = get_supported_symbols(&config.api_key)?;

    tracing::debug!("{:?}", &symbols);
    tracing::info!("{} Symbols updated", symbols.len());

    let path = Path::new("./symbols-supported.tsv");

    create_or_update_file(&symbols, path)?;

    Ok(())
}

// TODO : Add Unit test
// TODO : Add Doc
fn create_or_update_file<T>(data: &Vec<T>, path: &Path) -> Result<()>
where
    T: Serialize,
{
    let mut wrt = csv::WriterBuilder::new().delimiter(b'\t').from_path(path)?;

    for row in data {
        wrt.serialize(row)?;
    }

    Ok(())
}

#[derive(Deserialize, Debug)]
struct SuccessLatestResponseAPI {
    //success: bool,
    //success: bool,
    //timestamp: i64,
    //base: String,
    //date: String,
    rates: HashMap<String, Decimal>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum LatestResponseAPI {
    Success(SuccessLatestResponseAPI),
    Fail(ErrorResponseAPI),
}

fn get_conversion_rates(api_key: &str, base: &str) -> Result<Vec<ConversionRate>> {
    let url = format!("http://api.exchangeratesapi.io/v1/latest?access_key={api_key}&base={base}");

    let response = reqwest::blocking::get(&url)?;

    match response.json()? {
        LatestResponseAPI::Success(s) => Ok(common::conversion_rate::convert_hashmap_to_vec(
            s.rates, base,
        )?),
        LatestResponseAPI::Fail(f) => Err(anyhow!(
            "Call {} failed : {} - {}",
            url,
            f.error.code,
            f.error.message
        )),
    }
}

#[derive(Deserialize, Debug)]
struct SuccessSymbolResponseAPI {
    //success: bool,
    symbols: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
struct ErrorResponseAPI {
    // success : bool,
    error: ErrorInfoAPI,
}

#[derive(Deserialize, Debug)]
struct ErrorInfoAPI {
    code: String,
    message: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum SymbolResponseAPI {
    Success(SuccessSymbolResponseAPI),
    Fail(ErrorResponseAPI),
}

fn get_supported_symbols(api_key: &str) -> Result<Vec<Symbols>> {
    let url = format!("http://api.exchangeratesapi.io/v1/symbols?access_key={api_key}");

    let response = reqwest::blocking::get(&url)?;

    match response.json()? {
        SymbolResponseAPI::Success(s) => Ok(
            commands::common::supported_symbols::from_hash_map_to_vec(s.symbols)?,
        ),
        SymbolResponseAPI::Fail(f) => Err(anyhow!(
            "Call {} failed : {} - {}",
            url,
            f.error.code,
            f.error.message
        )),
    }
}
