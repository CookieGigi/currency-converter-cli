use std::{collections::HashMap, path::Path};

use anyhow::anyhow;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::*;

use self::config::Config;

use super::common::{
    conversion_rate::ConversionRate,
    supported_symbols::{from_hash_map_to_vec, Symbols},
};

pub fn run_update(config: Config) -> Result<()> {
    // symbols
    let symbols = get_supported_symbols(&config.api_key)?;

    tracing::debug!("{:?}", &symbols);
    tracing::info!("{} Symbols updated", symbols.len());

    let data = get_conversion_rates(&symbols)?;

    let path = Path::new("./currency-conversion-rates.tsv");

    create_or_update_file(data, path)?;
    Ok(())
}

// TODO : Add Unit test
// TODO : Add Doc
fn create_or_update_file<T>(data: Vec<T>, path: &Path) -> Result<()>
where
    T: Serialize,
{
    let mut wrt = csv::Writer::from_path(path)?;

    for row in data {
        wrt.serialize(row)?;
    }

    Ok(())
}

#[derive(Deserialize, Debug)]
struct SuccessLatestResponseAPI {
    //success: bool,
    symbols: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum LatestResponseAPI {
    Success(SuccessSymbolResponseAPI),
    Fail(ErrorResponseAPI),
}

fn get_conversion_rates(symbols: &Vec<Symbols>) -> Result<Vec<ConversionRate>> {
    //let data = Vec::new();

    let data = vec![ConversionRate {
        from: "EUR".to_string(),
        to: "USD".to_string(),
        rate: Decimal::new(108, 2),
    }];

    Ok(data)
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
        SymbolResponseAPI::Success(s) => Ok(from_hash_map_to_vec(s.symbols)?),
        SymbolResponseAPI::Fail(f) => Err(anyhow!(
            "Call {} failed : {} - {}",
            url,
            f.error.code,
            f.error.message
        )),
    }
}
