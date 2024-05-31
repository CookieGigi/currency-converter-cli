use currency_conversion::storage::common::DataInfo;

use super::info_config::ConfigInfo;

/// All type of information
#[derive(Debug)]
pub enum Info {
    Config(ConfigInfo),
    Symbols(DataInfo),
    ConversionRates(DataInfo),
}
