use anyhow::Result;
use cli::SubCommand;

pub mod cli;
pub mod config;
pub mod errors;

pub fn run(sub_command: SubCommand) -> Result<()> {
    match sub_command {
        SubCommand::Update => run_update()?,
    }
    Ok(())
}

fn run_update() -> Result<()> {
    Ok(())
}
