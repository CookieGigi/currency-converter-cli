use anyhow::Result;

use super::common::{get_data_info, DataInfo};
use currency_conversion::common::supported_symbols::Symbols;

/// Get informations about symbols data store locally
#[cfg(not(tarpaulin_include))]
pub fn get_symbols_info(path: &str) -> Result<DataInfo> {
    get_data_info::<Symbols>(path)
}
