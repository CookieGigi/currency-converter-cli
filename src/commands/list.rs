use anyhow::Result;
use currency_conversion::common::conversion_rate::ConversionRate;
use currency_conversion::common::load_data;
use currency_conversion::common::supported_symbols::Symbols;
use currency_conversion::list::list_data::list_data;
use currency_conversion::list::list_data::ListDataItem;
use serde::Deserialize;
use std::path::Path;

use crate::{
    cli::{ListArgs, ListDataSet},
    config::Config,
};

#[cfg(not(tarpaulin_include))]
pub fn run_list(config: &Config, args: &ListArgs) -> Result<()> {
    match args.dataset {
        ListDataSet::Symbols => load_and_list_data::<Symbols>(&config.symbols_file_path)?,
        ListDataSet::ConversionRates => {
            load_and_list_data::<ConversionRate>(&config.conversion_rates_file_path)?
        }
    };
    Ok(())
}

#[cfg(not(tarpaulin_include))]
fn load_and_list_data<T>(path: &str) -> Result<()>
where
    T: ListDataItem + for<'de> Deserialize<'de> + Ord,
{
    let mut data: Vec<T> = load_data(Path::new(&path))?;

    data.sort();

    println!("{}", list_data(&data)?);

    Ok(())
}
