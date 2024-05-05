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
pub fn run(sub_command: SubCommand, config: Config) -> Result<()> {
    use commands::convert::run_convert;

    match sub_command {
        SubCommand::Update => run_update(config)?,
        SubCommand::Convert(args) => run_convert(config, args)?,
    }
    Ok(())
}
