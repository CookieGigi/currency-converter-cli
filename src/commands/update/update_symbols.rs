use std::{collections::HashMap, path::Path};

use crate::{
    commands::{common::supported_symbols::Symbols, update::common::create_or_update_file},
    config::Config,
};

use anyhow::Result;
use serde::Deserialize;

use super::common::ErrorResponseAPI;

/// Update supported symbols file
pub fn update_symbols(config: &Config) -> Result<()> {
    // symbols
    let symbols = get_supported_symbols(&config.api_key)?;

    tracing::debug!("{:?}", &symbols);
    tracing::info!("{} Symbols updated", symbols.len());

    let path = Path::new("./symbols-supported.tsv");

    create_or_update_file(&symbols, path)?;

    Ok(())
}

/// Exchange rates API success response on symbols endpoint
#[derive(Deserialize, Debug)]
struct SuccessSymbolResponseAPI {
    //success: bool,
    symbols: HashMap<String, String>,
}

/// Exchange rates API response on symbols endpoint
#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum SymbolResponseAPI {
    Success(SuccessSymbolResponseAPI),
    Fail(ErrorResponseAPI),
}

/// Get supported symbols from exchanges rates API
fn get_supported_symbols(api_key: &str) -> Result<Vec<Symbols>> {
    let url = format!("http://api.exchangeratesapi.io/v1/symbols?access_key={api_key}");

    let response = reqwest::blocking::get(&url)?;

    match response.json()? {
        SymbolResponseAPI::Success(s) => {
            Ok(crate::commands::common::supported_symbols::from_hash_map_to_vec(s.symbols)?)
        }
        SymbolResponseAPI::Fail(f) => Err(anyhow::anyhow!(
            "Call {} failed : {} - {}",
            url,
            f.error.code,
            f.error.message
        )),
    }
}
