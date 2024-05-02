use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Symbols {
    pub code: String,
    pub name: String,
}

pub fn from_hash_map_to_vec(data: HashMap<String, String>) -> Result<Vec<Symbols>> {
    let mut res: Vec<Symbols> = Vec::new();

    for (key, value) in data.into_iter() {
        res.push(Symbols {
            code: key.clone(),
            name: value.clone(),
        });
    }

    Ok(res)
}
