use anyhow::Result;

use crate::commands::common::conversion_rate::ConversionRate;

use super::common::{get_data_info, DataInfo};

/// Get informations about conversion rate data store locally
#[cfg(not(tarpaulin_include))]
pub fn get_converison_rates_info(path: &str) -> Result<DataInfo> {
    get_data_info::<ConversionRate>(path)
}
