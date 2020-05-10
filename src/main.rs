//! Nib
//!
//! # Examples
//!
//! ```zsh
//! % nib
//! ```
use std::fs;
use std::io::{Error, ErrorKind};
use std::panic::{self, AssertUnwindSafe};
use std::path::Path;
use std::process;

use libnib::config::Config;
use libnib::fs::{get_entries, rem_results};
use libnib::registry::{add_escape_fn, del_escape_fn, init_registry};
use libnib::writer::{make_index, move_entry, save_entry};

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

fn cleanup() -> Result<(), Error> {
    let dst_dir = Path::new(DST_DIR);
    if dst_dir.exists() {
        let ptrn = dst_dir.join("*.html");
        let dst = ptrn
            .to_str()
            .ok_or_else(|| Error::new(ErrorKind::Other, "no pattern"))?;
        rem_results(dst)?;
    }
    fs::create_dir_all(Path::new(DST_DIR).join("css"))?;
    fs::create_dir_all(Path::new(DST_DIR).join("js"))?;
    fs::create_dir_all(Path::new(DST_DIR).join("img"))?;
    Ok(())
}

fn configure() -> Result<Config, Error> {
    let s = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&s)?;
    Ok(config)
}

fn setup() -> Result<(), Error> {
    run(|| {
        cleanup()?;
        Ok(())
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

    let config = configure()?;
    let result = run(|| {
        let mut ptrn;

        // TODO: take the paths from include
        let src_dir = Path::new(SRC_DIR);
        if !src_dir.exists() {
            return Err(Error::new(ErrorKind::NotFound, "no src directory"));
        }
        ptrn = src_dir.join("*.rst");
        let src = ptrn
            .to_str()
            .ok_or_else(|| Error::new(ErrorKind::Other, "no pattern"))?;

        let mut dat: Vec<_> = vec![];

        for e in get_entries(src).filter_map(Result::ok) {
            let info = save_entry(&e, &reg, &config)?;
            dat.push(info);
        }
        make_index(&mut dat, &reg, &config)?;

        // TODO: static
        let dir = Path::new(file!()).parent().expect("can't get a directory");
        ptrn = Path::new(dir)
            .join("theme")
            .join("static")
            .join("*")
            .join("*");
        let stc = ptrn
            .to_str()
            .ok_or_else(|| Error::new(ErrorKind::Other, "no pattern"))?;

        let dst = config.build.get_target_dir();
        let dst_dir = Path::new(&dst);
        for s in get_entries(&stc).filter_map(Result::ok) {
            if !s.is_file() {
                continue;
            }
            let ext = s.extension().map_or_else(|| "", |e| e.to_str().unwrap());
            if ext == "css" {
                let to = dst_dir.join("css");
                move_entry(&s, &to)?;
            }
        }
        Ok(())
    });

    del_escape_fn(&mut reg);
    teardown()?;

    if result.is_err() {
        eprintln!("{}", result.err().unwrap());
        process::exit(1);
    }

    Ok(())
}
