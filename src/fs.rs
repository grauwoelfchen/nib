//!
use std::fs;
use std::io::Error;

use glob::glob;

/// fetches given src entries.
pub fn get_entries(path: &str) -> glob::Paths {
    glob(path).expect("failed to read glob pattern")
}

/// deletes generated results.
pub fn rem_results(path: &str) -> Result<(), Error> {
    let results = glob(path).expect("failed to read glob pattern");
    for f in results.filter_map(std::result::Result::ok) {
        fs::remove_file(f)?;
    }
    Ok(())
}
