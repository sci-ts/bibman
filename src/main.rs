mod entry;
use std::path::PathBuf;
use entry::Entry;


fn main() {
    let entry_test_file: PathBuf = ["test-data", "entry.toml"].iter().collect();
    let entry = Entry::from_file(&entry_test_file);
    println!("{:?}", entry);
}
