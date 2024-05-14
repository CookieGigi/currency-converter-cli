use anyhow::Result;

pub trait ListDataItem {
    /// Define how write item in a list
    fn display_item(&self) -> String;
}

/// List all items using [`ListDataItem`] in a String with "\n" between each item
// Ignore in tarpaulin because it detect uncovered line on iter() for some reason
#[cfg(not(tarpaulin_include))]
pub fn list_data<T>(data: &[T]) -> Result<String>
where
    T: ListDataItem,
{
    Ok(data
        .iter()
        .fold(String::new(), |acc, item| acc + &item.display_item() + "\n"))
}

#[cfg(test)]
mod test {

    use super::ListDataItem;

    struct TestData {
        code: u8,
        name: String,
    }

    impl ListDataItem for TestData {
        fn display_item(&self) -> String {
            format!("{} : {}", self.code, self.name)
        }
    }

    #[test]
    fn list_data() {
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

        let expected = "1 : 1\n2 : 2\n";

        let res = super::list_data(&data);

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), expected);
    }
}
