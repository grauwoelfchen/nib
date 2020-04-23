//! Beta
//!
//! # Examples
//!
//! ```zsh
//! % beta
//! ```
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

use beta::{get_entries, generate_entry, load_registry};

const SRC_DIR: &str = "blog";
const DST_DIR: &str = "dst";

fn main() -> std::io::Result<()> {
    let dst = Path::new(DST_DIR);
    if dst.exists() {
        fs::remove_dir_all(DST_DIR)?;
    }
    fs::create_dir_all(DST_DIR)?;

    let ptrn = Path::new(SRC_DIR).join("*.rst");
    let path = ptrn
        .to_str()
        .ok_or_else(|| Error::new(ErrorKind::Other, "Unexpected source"))?;

    let mut reg = load_registry();
    for e in get_entries(path).filter_map(std::result::Result::ok) {
        generate_entry(&e, &mut reg, DST_DIR)?;
    }
    Ok(())
}
