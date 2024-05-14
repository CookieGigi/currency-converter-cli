use crate::config::Config;

use anyhow::Result;
use currency_conversion::update::{
    update_converison_rates::update_conversion_rates, update_symbols::update_symbols,
};

#[cfg(not(tarpaulin_include))]
pub fn run_update(config: &Config) -> Result<()> {
    update_symbols(
        &config.symbols_endpoint_url,
        &config.api_key,
        &config.symbols_file_path,
    )?;

    update_conversion_rates(
        &config.latest_endpoint_url,
        &config.api_key,
        &config.base,
        &config.conversion_rates_file_path,
    )?;
    Ok(())
}
