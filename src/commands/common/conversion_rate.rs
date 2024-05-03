use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use anyhow::Result;

/// Conversion Rates from a currency to another
#[derive(Serialize, Deserialize)]
pub struct ConversionRate {
    pub from: String,
    pub to: String,
    pub rate: Decimal,
}

/// Convert a hashmap (key : to currency, value : rates) and base (from currency) to vec of ConversionRate
pub fn from_hash_map_to_vec(
    data: HashMap<String, Decimal>,
    base: &str,
) -> Result<Vec<ConversionRate>> {
    let mut res: Vec<ConversionRate> = Vec::new();

    for (key, value) in data.into_iter() {
        res.push(ConversionRate {
            from: base.to_string(),
            to: key,
            rate: value,
        });
    }

    Ok(res)
}
