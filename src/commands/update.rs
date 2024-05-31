use crate::config::Config;

use crate::cli::UpdateArgs;
use anyhow::Result;
use currency_conversion::update::{
    update_converison_rates::update_conversion_rates, update_symbols::update_symbols,
};

#[cfg(not(tarpaulin_include))]
pub fn run_update(config: &Config, args: &UpdateArgs) -> Result<()> {
    use currency_conversion::storage::common::{
        get_conversion_rate_storage_manager, get_symbols_storage_manager,
    };

    tracing::debug!("Update arguments : {:?}", args);

    if args.all || args.symbols {
        tracing::info!("Update symbols begin");

        let storage_manager = get_symbols_storage_manager(config.symbols_storage.clone());

        update_symbols(
            &config.symbols_endpoint_url,
            &config.api_key,
            &storage_manager,
        )?;

        tracing::info!("Update symbols end");
    }

    if args.all || args.conversion_rates {
        tracing::info!("Update conversion rates begin");

        let storage_manager =
            get_conversion_rate_storage_manager(config.conversion_rates_storage.clone());

        update_conversion_rates(
            &config.latest_endpoint_url,
            &config.api_key,
            &config.base,
            &storage_manager,
        )?;
        tracing::info!("Update conversion rates end");
    }
    Ok(())
}
