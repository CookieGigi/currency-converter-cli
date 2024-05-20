use anyhow::Result;
use cli::SubCommand;
use commands::update::run_update;
use config::Config;

pub mod cli;
pub mod commands;
pub mod config;
pub mod errors;

/// Handle commands execution
#[cfg(not(tarpaulin_include))]
pub fn run(sub_command: SubCommand, config: Config, config_path: Option<String>) -> Result<()> {
    use commands::convert::run_convert;

    use crate::commands::{config::run_config, info::run_info, list::run_list};

    match sub_command {
        SubCommand::Update => run_update(&config)?,
        SubCommand::Convert(args) => run_convert(&config, &args)?,
        SubCommand::List(args) => run_list(&config, &args)?,
        SubCommand::Info(args) => run_info(config, &args, config_path)?,
        SubCommand::Config => run_config(&config, &config_path)?,
    }
    Ok(())
}
