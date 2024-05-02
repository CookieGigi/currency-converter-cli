use anyhow::Result;
use cli::SubCommand;
use commands::update::run_update;
use config::Config;

pub mod cli;
pub mod commands;
pub mod config;
pub mod errors;

pub fn run(sub_command: SubCommand, config: Config) -> Result<()> {
    match sub_command {
        SubCommand::Update => run_update(config)?,
    }
    Ok(())
}
