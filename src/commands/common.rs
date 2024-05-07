use serde::{Deserialize, Serialize};
use std::path::Path;
use anyhow::Result;
pub mod conversion_rate;
pub mod supported_symbols;
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
