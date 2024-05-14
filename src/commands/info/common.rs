use std::{path::Path, time::Duration};

use serde::{Deserialize, Serialize};

use crate::commands::common::load_data;

use super::info_config::ConfigInfo;

use anyhow::Result;

#[derive(Serialize, Debug)]
pub enum Info {
    Config(ConfigInfo),
    Symbols(DataInfo),
    ConversionRates(DataInfo),
}

#[derive(Serialize, Debug)]
pub struct DataInfo {
    pub seconds_since_last_update: Duration,
    pub number_of_line: usize,
}

pub fn get_data_info<T>(path: &str) -> Result<DataInfo>
where
    T: for<'de> Deserialize<'de>,
{
    let file_metadata = std::fs::metadata(path)?;

    let file_data = load_data::<T>(Path::new(path))?;

    Ok(DataInfo {
        seconds_since_last_update: file_metadata.modified()?.elapsed()?,
        number_of_line: file_data.len(),
    })
}
