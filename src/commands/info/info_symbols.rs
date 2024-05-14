use anyhow::Result;

use crate::commands::common::supported_symbols::Symbols;

use super::common::{get_data_info, DataInfo};

pub fn get_symbols_info(path: &str) -> Result<DataInfo> {
    Ok(get_data_info::<Symbols>(path)?)
}
