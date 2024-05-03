use std::path::Path;

use serde::{Deserialize, Serialize};

use anyhow::Result;

/// Exchange rates API error response
#[derive(Deserialize, Debug)]
pub struct ErrorResponseAPI {
    // success : bool,
    pub error: ErrorInfoAPI,
}

/// Exchange rates API error information
#[derive(Deserialize, Debug)]
pub struct ErrorInfoAPI {
    pub code: String,
    pub message: String,
}

/// Write a Vec in a tsv file
pub fn create_or_update_file<T>(data: &Vec<T>, path: &Path) -> Result<()>
where
    T: Serialize,
{
    let mut wrt = csv::WriterBuilder::new().delimiter(b'\t').from_path(path)?;

    for row in data {
        wrt.serialize(row)?;
    }

    Ok(())
}
