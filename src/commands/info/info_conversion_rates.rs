use anyhow::Result;

use super::common::{get_data_info, DataInfo};
use currency_conversion::common::conversion_rate::ConversionRate;

/// Get informations about conversion rate data store locally
#[cfg(not(tarpaulin_include))]
pub fn get_converison_rates_info(path: &str) -> Result<DataInfo> {
    get_data_info::<ConversionRate>(path)
}
