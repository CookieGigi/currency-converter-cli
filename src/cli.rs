//! Cli Arguments Parsing

use clap::Parser;
use clap_verbosity_flag::Verbosity;

/// Cli arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[command(flatten)]
    pub verbose: Verbosity,
}
