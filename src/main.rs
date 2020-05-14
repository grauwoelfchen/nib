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
use libnib::fs::{get_entries, rem_results, to_child_str_path};
use libnib::registry::{add_escape_fn, del_escape_fn, init_registry};
use libnib::writer::{make_index, move_entry, save_entry};

// TODO: refactor errors

fn run<B>(block: B) -> Result<(), Error>
where B: FnOnce() -> Result<(), Error> {
    let result = panic::catch_unwind(AssertUnwindSafe(|| block()));
    if result.is_err() {
        return Err(Error::new(ErrorKind::Other, "error"));
    }
    Ok(())
}

fn cleanup(c: &Config) -> Result<(), Error> {
    let dir = c.build.get_target_dir();
    let dst_dir = Path::new(&dir);
    if dst_dir.exists() {
        let ptrn = dst_dir.join("*.html");
        let dst = ptrn
            .to_str()
            .ok_or_else(|| Error::new(ErrorKind::Other, "no pattern"))?;
        rem_results(dst)?;
    }
    fs::create_dir_all(dst_dir.join("css"))?;
    fs::create_dir_all(dst_dir.join("js"))?;
    fs::create_dir_all(dst_dir.join("img"))?;
    Ok(())
}

fn configure() -> Result<Config, Error> {
    let s = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&s)?;
    Ok(config)
}

fn setup(c: &Config) -> Result<(), Error> {
    run(|| {
        cleanup(c)?;
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

    let config = configure()?;
    setup(&config)?;

    let result = run(|| {
        let src = &config.website.get_include();
        let mut dat: Vec<_> = vec![];

        let target = config.build.get_target_dir();
        let target_dir = Path::new(&target);

        for e in get_entries(src.to_vec()) {
            let path = e
                .as_path()
                .parent()
                .ok_or_else(|| Error::new(ErrorKind::NotFound, ""))?;
            if path.is_dir() {
                let dir = to_child_str_path(&path);
                if !dir.is_empty() {
                    let dst = target_dir.join(dir);
                    fs::create_dir_all(&dst)?;
                    let info = save_entry(&e, &dst, &reg, &config)?;
                    dat.push(info);
                    continue;
                }
            }
            // files which are put directly under the dst directory
            let info = save_entry(&e, &target_dir, &reg, &config)?;
            dat.push(info);
        }
        make_index(&mut dat, &reg, &config)?;

        // TODO: static from given theme
        let module_dir =
            Path::new(file!()).parent().expect("can't get a directory");
        let static_dir = vec![Path::new(module_dir)
            .join("theme")
            .join("static")
            .join("*")
            .join("*")
            .to_string_lossy()
            .into_owned()];

        for s in get_entries(static_dir) {
            if !s.is_file() {
                continue;
            }
            let ext = s.extension().map_or_else(|| "", |e| e.to_str().unwrap());
            if ext == "css" {
                let to = target_dir.join("css");
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
