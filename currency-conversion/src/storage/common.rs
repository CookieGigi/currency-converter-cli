use std::time::Duration;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::common::{conversion_rate::ConversionRate, supported_symbols::Symbols};

use super::tsv::{TSVStorageManager, TSVStorageSettings};

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
pub enum StorageType {
    TSV(TSVStorageSettings),
}

/// Information about data
#[derive(Debug)]
pub enum DataInfo {
    Success(DataInfoSuccess),
    Error(DataInfoError),
}

#[derive(Debug)]
pub struct DataInfoSuccess {
    pub seconds_since_last_update: Duration,
    pub number_of_line: usize,
}

#[derive(Debug)]
pub struct DataInfoError {
    pub error: anyhow::Error,
}

pub trait StorageManager<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    fn update(&self, data: &[T]) -> Result<()>;

    fn get_all(&self) -> Result<Vec<T>>;

    fn get_data_info(&self) -> Result<DataInfo>;
}

pub fn get_conversion_rate_storage_manager(
    storage_type: StorageType,
) -> impl StorageManager<ConversionRate> {
    match storage_type {
        StorageType::TSV(settings) => TSVStorageManager::from_settings(settings),
    }
}

pub fn get_symbols_storage_manager(storage_type: StorageType) -> impl StorageManager<Symbols> {
    match storage_type {
        StorageType::TSV(settings) => TSVStorageManager::from_settings(settings),
    }
}
