//! Cli Arguments Parsing

use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use rust_decimal::Decimal;

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

    /// Optional : config profile name
    #[arg(long)]
    pub config_profile: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// Update supported symbols and conversion rates files
    Update(UpdateArgs),
    /// Convert a value from a currency to an other
    Convert(ConvertArgs),
    /// List all data from dataset indicated
    List(ListArgs),
    /// Show informations
    Info(InfoArgs),
    /// Prompt config
    Config,
}

#[derive(Args, Debug)]
pub struct ConvertArgs {
    /// origin currency
    #[arg(long)]
    pub from: String,
    /// destination currency
    #[arg(long)]
    pub to: String,
    /// value to convert
    pub value: Decimal,
}

#[derive(Args, Debug)]
pub struct ListArgs {
    /// dataset to List
    #[command(subcommand)]
    pub dataset: ListDataSet,
}

#[derive(Debug, Subcommand)]
pub enum ListDataSet {
    Symbols,
    ConversionRates,
}

#[derive(Args, Debug)]
pub struct InfoArgs {
    /// show all information
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub all: bool,

    /// show config information
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub config: bool,

    /// show symbols information
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub symbols: bool,

    /// show symbols information
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub conversion_rates: bool,
}

#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Update all
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub all: bool,
    /// Update symbols
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub symbols: bool,
    /// Update conversion rates
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub conversion_rates: bool,
}
