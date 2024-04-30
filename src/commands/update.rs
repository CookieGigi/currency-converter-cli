use std::fs::File;

use rust_decimal::Decimal;

use crate::*;

use super::common::conversion_rate::ConversionRate;

pub fn run_update() -> Result<()> {
    let data = vec![ConversionRate {
        from: "EUR".to_string(),
        to: "USD".to_string(),
        rate: Decimal::new(189, 2),
    }];

    create_or_update_file(data)?;
    Ok(())
}

fn create_or_update_file(data: Vec<ConversionRate>) -> Result<()> {
    let file = File::create("./currency-conversion-rates.tsv");

    Ok(())
}
