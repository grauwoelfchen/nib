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

use libbeta::writer::{write_entry, make_index};
use libbeta::registry::{add_escape_fn, rem_escape_fn, load_registry};
use libbeta::file::{get_entries, rem_results};

const SRC_DIR: &str = "blog";
const DST_DIR: &str = "dst";

fn main() -> std::io::Result<()> {
    let mut ptrn;

    let dst_dir = Path::new(DST_DIR);
    if dst_dir.exists() {
        ptrn = dst_dir.join("*.html");
        let dst = ptrn
            .to_str()
            .ok_or_else(|| Error::new(ErrorKind::Other, "unexpected dst"))?;
        rem_results(dst)?;
    }
    fs::create_dir_all(DST_DIR)?;

    let src_dir = Path::new(SRC_DIR);
    if !src_dir.exists() {
        return Err(Error::new(ErrorKind::NotFound, "no src directory"));
    }
    ptrn = src_dir.join("*.rst");
    let src = ptrn
        .to_str()
        .ok_or_else(|| Error::new(ErrorKind::Other, "unexpected src"))?;

    let mut reg = load_registry().expect("");
    let mut dat: Vec<_> = vec![];

    add_escape_fn(&mut reg);
    for e in get_entries(src).filter_map(std::result::Result::ok) {
        let info = write_entry(&e, &reg, DST_DIR)?;
        dat.push(info);
    }

    make_index(&mut dat, &reg, DST_DIR)?;
    rem_escape_fn(&mut reg);
    Ok(())
}
