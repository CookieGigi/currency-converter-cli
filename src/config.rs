use serde::{Deserialize, Serialize};

/// Config file structure
#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Config {
    /// API token
    pub api_key: String,
    /// base currency
    pub base: String,
    /// endpoint url to get supported symbols (param : {api_key})
    pub symbols_endpoint_url: String,
    /// endpoint url to get conversion rates (param : {api_key}, {base})
    pub latest_endpoint_url: String,
    /// file path where supported symbols are stored
    pub symbols_file_path: String,
    // file path where conversion rates are stored
    pub conversion_rates_file_path: String,
}
