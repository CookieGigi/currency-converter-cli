use std::path::Path;

use anyhow::Result;
use serde::Deserialize;

use crate::{
    cli::{ListArgs, ListDataSet},
    commands::list::list_data::list_data,
    config::Config,
};

use self::list_data::ListDataItem;

use super::common::{conversion_rate::ConversionRate, load_data, supported_symbols::Symbols};

mod list_data;

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

impl ListDataItem for ConversionRate {
    #[cfg(not(tarpaulin_include))]
    fn display_item(&self) -> String {
        format!("{} -> {} : {}", &self.from, &self.to, &self.rate)
    }
}

impl ListDataItem for Symbols {
    #[cfg(not(tarpaulin_include))]
    fn display_item(&self) -> String {
        format!("{} : {}", &self.code, &self.name)
    }
}
