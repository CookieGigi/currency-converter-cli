//! Cli Arguments Parsing

use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

/// Cli arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[command(flatten)]
    pub verbose: Verbosity,

    #[command(subcommand)]
    pub sub_command: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    // TODO : Document
    Update,
}
