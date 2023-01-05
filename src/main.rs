mod bibliography;
mod entry;
use bibliography::Bibliography;
use std::path::PathBuf;

fn main() {
    let bib_test_file: PathBuf = ["test-data", "bibliography.toml"].iter().collect();
    let bib = Bibliography::from_file(&bib_test_file).unwrap();
    println!("{:?}", bib);
    bib.to_file(&bib_test_file).unwrap();
}
