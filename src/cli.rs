//! Cli Arguments Parsing

use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

/// Handle currency conversion using local saved conversion rates
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[command(flatten)]
    pub verbose: Verbosity,

    #[command(subcommand)]
    pub sub_command: SubCommand,

    /// Optional : path to config file (default : handle by confy)
    #[arg(long)]
    pub config_path: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// Update supported symbols and conversion rates files
    Update,
}
