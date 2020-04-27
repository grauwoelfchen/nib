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

use libbeta::{
    add_escape_fn, generate_entry, generate_index, get_entries, load_registry,
    rem_escape_fn, Variables,
};

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
    let mut dat: Vec<Variables> = vec![];

    add_escape_fn(&mut reg);
    for e in get_entries(path).filter_map(std::result::Result::ok) {
        let info = generate_entry(&e, &reg, DST_DIR)?;
        dat.push(info);
    }
    generate_index(&mut dat, &reg, DST_DIR)?;

    rem_escape_fn(&mut reg);
    Ok(())
}
