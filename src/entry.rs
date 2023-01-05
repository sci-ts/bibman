use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum EntryKind {
    Article,
    Book,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    kind: EntryKind,

    pub authors: Option<Vec<String>>,
    pub title: Option<String>,
    pub journal: Option<String>,
    pub year: Option<u32>,

    pub doi: Option<String>,
    pub volume: Option<u32>,
    pub pages: Option<String>,
    pub month: Option<u32>,
}

impl Entry {
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
