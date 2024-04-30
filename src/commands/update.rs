use std::path::Path;

use rust_decimal::Decimal;

use crate::*;

use super::common::conversion_rate::ConversionRate;

pub fn run_update() -> Result<()> {
    let data = vec![ConversionRate {
        from: "EUR".to_string(),
        to: "USD".to_string(),
        rate: Decimal::new(189, 2),
    }];

    let path = Path::new("./currency-conversion-rates.tsv");

    create_or_update_file(data, path)?;
    Ok(())
}

// TODO : Add Unit test
// TODO : Add Doc
fn create_or_update_file(data: Vec<ConversionRate>, path: &Path) -> Result<()> {
    let mut wrt = csv::Writer::from_path(path)?;

    for row in data {
        wrt.serialize(row)?;
    }

    Ok(())
}
