use crate::{cli::ConvertArgs, config::Config};

use anyhow::Result;

use currency_conversion::convert::convert_currency::convert;

#[cfg(not(tarpaulin_include))]
pub fn run_convert(config: &Config, args: &ConvertArgs) -> Result<()> {
    let res = convert(
        &config.conversion_rates_file_path,
        &config.base,
        &args.from,
        &args.to,
        args.value,
    )?;

    println!("{res}");
    Ok(())
}
