use anyhow::Result;

/// Error handling function call in main
#[cfg(not(tarpaulin_include))]
pub fn errors_handling(error: anyhow::Error) -> Result<()> {
    tracing::error!("{:#}", error);

    // Return different exitcode depending on error type
    if error.downcast_ref::<std::io::Error>().is_some() {
        std::process::exit(exitcode::IOERR);
    }

    std::process::exit(1);
}
