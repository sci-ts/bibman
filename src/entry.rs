use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
