use crate::config::Config;

use anyhow::Result;

use self::update_converison_rates::update_conversion_rates;

mod common;
mod update_converison_rates;
mod update_symbols;

pub fn run_update(config: Config) -> Result<()> {
    update_symbols::update_symbols(&config)?;

    update_conversion_rates(&config)?;
    Ok(())
}
