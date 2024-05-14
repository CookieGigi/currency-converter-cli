use anyhow::Result;

use crate::commands::common::conversion_rate::ConversionRate;

use super::common::{get_data_info, DataInfo};

pub fn get_converison_rates_info(path: &str) -> Result<DataInfo> {
    Ok(get_data_info::<ConversionRate>(path)?)
}
