use std::{path::Path, time::Duration};

use serde::{Deserialize, Serialize};

use crate::commands::common::load_data;

use super::info_config::ConfigInfo;

use anyhow::Result;

/// All type of information
#[derive(Serialize, Debug)]
pub enum Info {
    Config(ConfigInfo),
    Symbols(DataInfo),
    ConversionRates(DataInfo),
}

/// Information about data
#[derive(Serialize, Debug)]
pub struct DataInfo {
    pub seconds_since_last_update: Duration,
    pub number_of_line: usize,
}

/// Get information about data
pub fn get_data_info<T>(path: &str) -> Result<DataInfo>
where
    T: for<'de> Deserialize<'de>,
{
    let file_metadata = std::fs::metadata(path)?;

    let file_data = load_data::<T>(Path::new(path))?;

    Ok(DataInfo {
        seconds_since_last_update: file_metadata.modified()?.elapsed()?,
        number_of_line: file_data.len(),
    })
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use serde::{Deserialize, Serialize};

    use crate::commands::common::create_or_update_file;

    #[derive(Serialize, Deserialize, Debug)]
    struct TestData {
        code: u8,
        name: String,
    }

    #[test]
    fn get_data_info() {
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

        let dirpath = "./temp/test/commands/info/common/";

        std::fs::create_dir_all(dirpath).unwrap();

        let path = dirpath.to_string() + "test.tsv";

        create_or_update_file(&data, Path::new(&path)).unwrap();

        let res = super::get_data_info::<TestData>(&path);

        assert!(res.is_ok());
        assert_eq!(res.unwrap().number_of_line, 2);

        std::fs::remove_dir_all(dirpath).unwrap();
    }
}
