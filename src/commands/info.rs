use std::collections::HashMap;

use anyhow::Result;

use crate::{
    cli::InfoArgs,
    commands::info::{
        info_conversion_rates::get_converison_rates_info, info_symbols::get_symbols_info,
    },
    config::Config,
};

use self::{common::Info, info_config::get_config_info};

mod common;
mod info_config;
mod info_conversion_rates;
mod info_symbols;

pub fn run_info(config: Config, args: &InfoArgs, config_path: Option<String>) -> Result<()> {
    let mut infos: HashMap<&str, Info> = HashMap::new();

    // Symbols
    if args.symbols || args.all {
        infos.insert(
            "symbols",
            Info::Symbols(get_symbols_info(&config.symbols_file_path)?),
        );
    }

    // Conversion rate

    if args.conversion_rates || args.all {
        infos.insert(
            "conversion_rates",
            Info::ConversionRates(get_converison_rates_info(
                &config.conversion_rates_file_path,
            )?),
        );
    }

    // Config
    if args.config || args.all {
        infos.insert(
            "config",
            Info::Config(get_config_info(config, config_path)?),
        );
    }

    println!("{:?}", infos);

    Ok(())
}
