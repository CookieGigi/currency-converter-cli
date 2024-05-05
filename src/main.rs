use anyhow::Result;
use clap::Parser;
use currency_converter_cli::{cli::CliArgs, config::Config, errors::errors_handling, run};
use tracing_log::AsTrace;

#[cfg(not(tarpaulin_include))]
fn main() -> Result<()> {
    // Get command line arguments
    let args = CliArgs::parse();

    // Get config
    let config: Config = match &args.config_path.is_none() {
        true => confy::load("currency-converter-cli", None)?,
        false => confy::load_path(args.config_path.unwrap())?,
    };

    // Initialize trace
    tracing_subscriber::fmt()
        .with_max_level(args.verbose.log_level_filter().as_trace())
        .init();

    match run(args.sub_command, config) {
        Err(error) => errors_handling(error),
        Ok(()) => {
            // Success Message
            tracing::info!("Success !");
            Ok(())
        }
    }
}
