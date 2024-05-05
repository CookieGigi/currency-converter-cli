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

#[cfg(test)]
mod test {
    use std::path::Path;

    use serde::Serialize;

    fn setup(path: &str) {
        std::fs::create_dir_all(path).unwrap();
    }

    fn end(path: &str) {
        std::fs::remove_dir_all(path).unwrap();
    }

    #[derive(Serialize)]
    struct TestStruct {
        code: u8,
        name: String,
    }

    #[test]
    fn create_or_update_file() {
        let dirpath = "./temp/test/commands/update/common";

        setup(dirpath);

        let data = vec![
            TestStruct {
                code: 1,
                name: "1".to_string(),
            },
            TestStruct {
                code: 2,
                name: "2".to_string(),
            },
        ];

        let path = dirpath.to_string() + "/test.tsv";

        assert!(super::create_or_update_file(&data, Path::new(&path)).is_ok());
        assert!(Path::new(&path).exists());

        end(dirpath);
    }
}
