use std::{collections::HashMap, path::Path};

use anyhow::Result;
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::{commands::common::conversion_rate::ConversionRate, config::Config};

use super::common::{create_or_update_file, ErrorResponseAPI};

/// Update conversion rate files
pub fn update_conversion_rates(config: &Config) -> Result<()> {
    let data = get_conversion_rates(&config.api_key, &config.base)?;

    let path = Path::new("./currency-conversion-rates.tsv");

    create_or_update_file(&data, path)?;

    Ok(())
}

/// Exchange rates API success response on latest endpoint
#[derive(Deserialize, Debug)]
struct SuccessLatestResponseAPI {
    //success: bool,
    //success: bool,
    //timestamp: i64,
    //base: String,
    //date: String,
    rates: HashMap<String, Decimal>,
}

/// Exchange rates API response on latest endpoint
#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum LatestResponseAPI {
    Success(SuccessLatestResponseAPI),
    Fail(ErrorResponseAPI),
}

/// Get conversion rates for the base currency to others currencies from exchanges rates API
fn get_conversion_rates(api_key: &str, base: &str) -> Result<Vec<ConversionRate>> {
    let url = format!("http://api.exchangeratesapi.io/v1/latest?access_key={api_key}&base={base}");

    let response = reqwest::blocking::get(&url)?;

    match response.json()? {
        LatestResponseAPI::Success(s) => {
            Ok(crate::commands::common::conversion_rate::from_hash_map_to_vec(s.rates, base)?)
        }
        LatestResponseAPI::Fail(f) => Err(anyhow::anyhow!(
            "Call {} failed : {} - {}",
            url,
            f.error.code,
            f.error.message
        )),
    }
}
