use std::{path::Path, time::Duration};

use serde::Deserialize;

use crate::commands::common::load_data;

use super::info_config::ConfigInfo;

use anyhow::Result;

/// All type of information
#[derive(Debug)]
pub enum Info {
    Config(ConfigInfo),
    Symbols(DataInfo),
    ConversionRates(DataInfo),
}

/// Information about data
#[derive(Debug)]
pub enum DataInfo {
    Success(DataInfoSuccess),
    Error(DataInfoError),
}

#[derive(Debug)]
pub struct DataInfoSuccess {
    pub seconds_since_last_update: Duration,
    pub number_of_line: usize,
}

#[derive(Debug)]
pub struct DataInfoError {
    pub error: std::io::Error,
}

/// Get information about data
pub fn get_data_info<T>(path: &str) -> Result<DataInfo>
where
    T: for<'de> Deserialize<'de>,
{
    let file_metadata = std::fs::metadata(path);

    match file_metadata {
        Ok(metadata) => {
            let file_data = load_data::<T>(Path::new(path))?;

            Ok(DataInfo::Success(DataInfoSuccess {
                seconds_since_last_update: metadata.modified()?.elapsed()?,
                number_of_line: file_data.len(),
            }))
        }
        Err(e) => Ok(DataInfo::Error(DataInfoError { error: e })),
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use serde::{Deserialize, Serialize};

    use crate::commands::{common::create_or_update_file, info::common::DataInfo};

    #[derive(Serialize, Deserialize, Debug)]
    struct TestData {
        code: u8,
        name: String,
    }

    #[test]
    fn get_data_info_success() {
        let data = vec![
            TestData {
                code: 1,
                name: "1".to_string(),
            },
            TestData {
                code: 2,
                name: "2".to_string(),
            },
        ];

        let dirpath = "./temp/test/commands/info/common/get_data_info_success";

        std::fs::create_dir_all(dirpath).unwrap();

        let path = dirpath.to_string() + "/test.tsv";

        create_or_update_file(&data, Path::new(&path)).unwrap();

        let res = super::get_data_info::<TestData>(&path);

        assert!(res.is_ok());

        assert!(matches!(res.unwrap(), DataInfo::Success(..)));

        std::fs::remove_dir_all(dirpath).unwrap();
    }

    #[test]
    fn get_data_info_error() {
        let dirpath = "./temp/test/commands/info/common/get_data_info_error";

        std::fs::create_dir_all(dirpath).unwrap();

        let path = dirpath.to_string() + "/test.tsv";

        let res = super::get_data_info::<TestData>(&path);

        assert!(res.is_ok());

        assert!(matches!(res.unwrap(), DataInfo::Error(..)));

        std::fs::remove_dir_all(dirpath).unwrap();
    }
}
