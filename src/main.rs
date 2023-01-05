mod bibliography;
mod entry;
use bibliography::Bibliography;
use std::path::PathBuf;

fn main() {
    let entry_test_file: PathBuf = ["test-data", "bibliography.toml"].iter().collect();
    let entry = Bibliography::from_file(&entry_test_file);
    println!("{:?}", entry);
}
