use anyhow::Result;

use crate::config::Config;

#[cfg(not(tarpaulin_include))]
pub fn run_config(config: &Config, config_path: &Option<String>) -> Result<()> {
    prompt_and_store_config(config, config_path)?;
    Ok(())
}

#[cfg(not(tarpaulin_include))]
pub fn prompt_and_store_config(config: &Config, config_path: &Option<String>) -> Result<Config> {
    let res = Config::prompt_config(config)?;
    match config_path.is_none() {
        true => confy::store("currency-converter-cli", None, &res)?,
        false => confy::store_path(config_path.clone().unwrap(), &res)?,
    };

    Ok(res)
}
