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

pub fn load_data<'a, T>(path: &Path) -> Result<Vec<T>>
where
    T: for<'de> Deserialize<'de>,
{
    let mut csv_rdr = csv::ReaderBuilder::new().delimiter(b'\t').from_path(path)?;

    let mut res = Vec::new();

    for line in csv_rdr.deserialize() {
        res.push(line?);
    }

    Ok(res)
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, PartialEq)]
    struct TestStruct {
        code: u8,
        name: String,
    }

    #[test]
    fn create_or_update_file() {
        let dirpath = "./temp/test/commands/update/common/create_or_update_file";

        std::fs::create_dir_all(dirpath).unwrap();

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

        std::fs::remove_dir_all(dirpath).unwrap();
    }

    #[test]
    fn load_data() {
        let data: Vec<TestStruct> = vec![
            TestStruct {
                code: 1,
                name: "1".to_string(),
            },
            TestStruct {
                code: 2,
                name: "2".to_string(),
            },
        ];

        let dirpath = "./temp/test/commands/update/common/load_data";

        std::fs::create_dir_all(dirpath).unwrap();

        let path = dirpath.to_string() + "/test2.tsv";

        super::create_or_update_file(&data, Path::new(&path)).unwrap();

        let res = super::load_data::<TestStruct>(Path::new(&path));

        assert!(res.is_ok());
        assert!(res.unwrap().contains(&data[0]));

        std::fs::remove_dir_all(dirpath).unwrap();
    }
}
