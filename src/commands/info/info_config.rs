use anyhow::Result;
use serde::Serialize;

use crate::config::Config;

/// Information about configuration of this app
#[derive(Serialize, Debug)]
pub struct ConfigInfo {
    config_path: String,
    content: Config,
}

/// Get informations about configuration of this app
pub fn get_config_info(config: Config, config_path: Option<String>) -> Result<ConfigInfo> {
    let current_config_path = match config_path {
        Some(path) => path,
        None => {
            let path = confy::get_configuration_file_path("currency-conversion-cli", None)?;

            let path = path.into_os_string().into_string();

            match path {
                Ok(p) => p,
                #[cfg(not(tarpaulin_include))]
                Err(os) => os.to_string_lossy().to_string(),
            }
        }
    };

    Ok(ConfigInfo {
        config_path: current_config_path,
        content: config,
    })
}

#[cfg(test)]
mod test {
    use crate::config::Config;

    #[test]
    fn get_config_info() {
        let config = Config {
            base: String::new(),
            api_key: String::new(),
            symbols_file_path: String::new(),
            latest_endpoint_url: String::new(),
            symbols_endpoint_url: String::new(),
            conversion_rates_file_path: String::new(),
        };
        let dirpath = "./temp/test/info/info_config/";

        std::fs::create_dir_all(dirpath).unwrap();

        let path = dirpath.to_string() + "config.toml";

        confy::store_path(path.clone(), &config).unwrap();

        let res = super::get_config_info(config, Some(path.clone()));

        assert!(res.is_ok());
        assert_eq!(res.unwrap().config_path, path);

        std::fs::remove_dir_all(dirpath).unwrap();
    }

    #[test]
    fn get_config_info_auto_path() {
        let config = Config {
            base: String::new(),
            api_key: String::new(),
            symbols_file_path: String::new(),
            latest_endpoint_url: String::new(),
            symbols_endpoint_url: String::new(),
            conversion_rates_file_path: String::new(),
        };

        confy::store("convert_currency_cli", "test", &config).unwrap();

        let res = super::get_config_info(config, None);

        assert!(res.is_ok());
    }
}
