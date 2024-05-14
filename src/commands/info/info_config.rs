use anyhow::Result;
use serde::Serialize;

use crate::config::Config;

#[derive(Serialize, Debug)]
pub struct ConfigInfo {
    config_path: String,
    content: Config,
}

pub fn get_config_info(config: Config, config_path: Option<String>) -> Result<ConfigInfo> {
    let current_config_path = match config_path {
        Some(path) => path,
        None => {
            let path = confy::get_configuration_file_path("", None)?;

            let path = path.into_os_string().into_string();

            match path {
                Ok(p) => p,
                Err(os) => os.to_string_lossy().to_string(),
            }
        }
    };

    Ok(ConfigInfo {
        config_path: current_config_path,
        content: config,
    })
}
