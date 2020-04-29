//! Beta
//!
//! # Examples
//!
//! ```zsh
//! % beta
//! ```
use std::fs;
use std::io::{Error, ErrorKind};
use std::panic::{self, AssertUnwindSafe};
use std::path::Path;

use libbeta::fs::{get_entries, rem_results};
use libbeta::registry::{add_escape_fn, del_escape_fn, init_registry};
use libbeta::writer::{write_entry, make_index};

const SRC_DIR: &str = "blog";
const DST_DIR: &str = "dst";

// TODO: refactor errors

fn run<B>(block: B) -> Result<(), Error>
where B: FnOnce() -> Result<(), Error> {
    let result = panic::catch_unwind(AssertUnwindSafe(|| block()));
    if result.is_err() {
        return Err(Error::new(ErrorKind::Other, "error"));
    }
    Ok(())
}

fn setup() -> Result<(), Error> {
    run(|| {
        let dst_dir = Path::new(DST_DIR);
        if dst_dir.exists() {
            let ptrn = dst_dir.join("*.html");
            let dst = ptrn
                .to_str()
                .ok_or_else(|| Error::new(ErrorKind::Other, "no pattern"))?;
            rem_results(dst)?;
        }
        fs::create_dir_all(DST_DIR)
    })
}

fn teardown() -> Result<(), Error> {
    run(|| {
        // TODO
        Ok(())
    })
}

fn main() -> Result<(), Error> {
    let mut reg = init_registry().expect("");

    add_escape_fn(&mut reg);
    setup()?;

    run(|| {
        let src_dir = Path::new(SRC_DIR);
        if !src_dir.exists() {
            return Err(Error::new(ErrorKind::NotFound, "no src directory"));
        }
        let ptrn = src_dir.join("*.rst");
        let src = ptrn
            .to_str()
            .ok_or_else(|| Error::new(ErrorKind::Other, "no pattern"))?;

        let mut dat: Vec<_> = vec![];

        for e in get_entries(src).filter_map(std::result::Result::ok) {
            let info = write_entry(&e, &reg, DST_DIR)?;
            dat.push(info);
        }
        make_index(&mut dat, &reg, DST_DIR)
    })?;

    del_escape_fn(&mut reg);
    teardown()?;

    Ok(())
}
