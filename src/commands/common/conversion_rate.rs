use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use anyhow::Result;

#[derive(Serialize, Deserialize)]
pub struct ConversionRate {
    pub from: String,
    pub to: String,
    pub rate: Decimal,
}

pub fn convert_hashmap_to_vec(
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
