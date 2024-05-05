use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use anyhow::Result;

/// Conversion Rates from a currency to another
#[derive(Serialize, Deserialize, PartialEq, Debug)]
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

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use rust_decimal::Decimal;

    use crate::commands::common::conversion_rate::ConversionRate;

    #[test]
    fn from_hash_map_to_vec() {
        let mut hashmap = HashMap::new();

        let base = "EUR".to_string();

        let usd = ConversionRate {
            from: base.clone(),
            to: "USD".to_string(),
            rate: Decimal::new(108, 2),
        };
        hashmap.insert("USD".to_string(), Decimal::new(108, 2));

        let tbh = ConversionRate {
            from: base.clone(),
            to: "TBH".to_string(),
            rate: Decimal::new(32, 0),
        };

        hashmap.insert("TBH".to_string(), Decimal::new(32, 0));

        let res = super::from_hash_map_to_vec(hashmap, &base).unwrap();

        assert!(res.contains(&usd));
        assert!(res.contains(&tbh));
    }
}
