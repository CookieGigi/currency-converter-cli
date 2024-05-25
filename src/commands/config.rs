use anyhow::Result;

use crate::config::Config;

#[cfg(not(tarpaulin_include))]
pub fn run_config(
    config: &Config,
    config_path: &Option<String>,
    config_profile: Option<&str>,
) -> Result<()> {
    prompt_and_store_config(config, config_path, config_profile)?;
    Ok(())
}

#[cfg(not(tarpaulin_include))]
pub fn prompt_and_store_config(
    config: &Config,
    config_path: &Option<String>,
    config_profile: Option<&str>,
) -> Result<Config> {
    let res = Config::prompt_config(config)?;
    match config_path.is_none() {
        true => confy::store("currency-converter-cli", config_profile, &res)?,
        false => confy::store_path(config_path.clone().unwrap(), &res)?,
    };

    println!("Config Initialized !");
    tracing::info!("Config Initialized !");
    tracing::debug!("{:?}", res);

    Ok(res)
}
