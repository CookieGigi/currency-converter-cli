use std::path::Path;

use rust_decimal::Decimal;

use crate::{
    commands::common::{conversion_rate::ConversionRate, load_data},
    config::Config,
};

use anyhow::Result;

pub fn convert(config: &Config, from: &str, to: &str, value: Decimal) -> Result<Decimal> {
    let conversion_rates = load_data(Path::new(&config.conversion_rates_file_path))?;

    let rate = ConversionRate::get_conversion_rate(config, &conversion_rates, from, to)?;

    Ok(value * rate.rate)
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use rust_decimal_macros::dec;

    use crate::{
        commands::common::{conversion_rate::ConversionRate, create_or_update_file},
        config::Config,
    };

    fn setup(dirpath: String, data: Vec<ConversionRate>) -> String {
        std::fs::create_dir_all(&dirpath).unwrap();

        let path = dirpath + "conversion_rate.tsv";

        create_or_update_file(&data, Path::new(&path)).unwrap();

        path
    }

    #[test]
    fn convert() {
        let dirpath = "./temp/test/convert/convert_currency/";
        let from = "EUR".to_string();
        let to = "USD".to_string();
        let data = vec![ConversionRate {
            from: from.clone(),
            to: to.clone(),
            rate: dec!(1.08),
        }];
        let filepath = setup(dirpath.to_string(), data);

        let config = Config {
            base: "EUR".to_string(),
            conversion_rates_file_path: filepath,
            ..Default::default()
        };

        let res = super::convert(&config, &from, &to, dec!(10.0));
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), dec!(10.8));
    }
}
