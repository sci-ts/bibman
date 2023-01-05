use std::{fs::OpenOptions, io::Write, path::Path};

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
                file.read_to_string(&mut buffer)
                    .expect("Could not read the file contents.");

                match toml::from_str(&buffer) {
                    Err(e) => Err(e.into()),
                    Ok(entry) => Ok(entry),
                }
            }
        }
    }

    pub fn to_file(&self, path: &Path) -> Result<(), std::io::Error> {
        let file = OpenOptions::new().create(true).write(true).open(path);

        match file {
            Err(e) => Err(e),
            Ok(mut file) => {
                let bib_str = toml::to_string_pretty(&self)
                    .expect("Could not write the bibliography to valid toml.");

                let parent_dir = path.parent().unwrap();
                std::fs::create_dir_all(parent_dir)
                    .expect("Could not create parent dirs for file.");
                file.write_all(bib_str.as_bytes())
                    .expect("Could not write data to a file");
                Ok(())
            }
        }
    }
}
