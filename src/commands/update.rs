use crate::config::Config;

use crate::cli::UpdateArgs;
use anyhow::Result;
use currency_conversion::update::{
    update_converison_rates::update_conversion_rates, update_symbols::update_symbols,
};

#[cfg(not(tarpaulin_include))]
pub fn run_update(config: &Config, args: &UpdateArgs) -> Result<()> {
    tracing::debug!("Update arguments : {:?}", args);

    if args.all || args.symbols {
        tracing::info!("Update symbols begin");

        update_symbols(
            &config.symbols_endpoint_url,
            &config.api_key,
            &config.symbols_file_path,
        )?;

        tracing::info!("Update symbols end");
    }

    if args.all || args.conversion_rates {
        tracing::info!("Update conversion rates begin");

        update_conversion_rates(
            &config.latest_endpoint_url,
            &config.api_key,
            &config.base,
            &config.conversion_rates_file_path,
        )?;
        tracing::info!("Update conversion rates end");
    }
    Ok(())
}
