use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ConversionRate {
    pub from: String,
    pub to: String,
    pub rate: Decimal,
}
