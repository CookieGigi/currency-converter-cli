use crate::{cli::ConvertArgs, config::Config};

use anyhow::Result;

use self::convert_currency::convert;

pub mod convert_currency;

#[cfg(not(tarpaulin_include))]
pub fn run_convert(config: Config, args: ConvertArgs) -> Result<()> {
    let res = convert(&config, &args.from, &args.to, args.value)?;

    println!("{res}");
    Ok(())
}
