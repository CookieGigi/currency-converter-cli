use rust_decimal::Decimal;

pub struct ConversionRate {
    pub from: String,
    pub to: String,
    pub rate: Decimal,
}
