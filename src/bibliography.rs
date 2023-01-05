use std::{fs::OpenOptions, path::Path};

use crate::entry::Entry;
use linked_hash_map::LinkedHashMap;
use serde::{Deserialize, Serialize};
use std::io::Read;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Bibliography {
    #[serde(flatten)]
    pub entries: LinkedHashMap<String, Entry>,
}

impl Bibliography {
    pub fn from_file(path: &Path) -> Result<Self, std::io::Error> {
        let file = OpenOptions::new().read(true).open(path);

        match file {
            Err(e) => Err(e),
            Ok(mut file) => {
                let mut buffer = String::new();
                file.read_to_string(&mut buffer).unwrap();

                match toml::from_str(&buffer) {
                    Err(e) => Err(e.into()),
                    Ok(entry) => Ok(entry),
                }
            }
        }
    }
}
