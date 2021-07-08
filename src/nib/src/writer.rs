//!
use std::fs;
use std::io::{Error, ErrorKind, Write};
use std::path::Path;

use handlebars::Handlebars;
use serde_json::Value;

use crate::include_static_file;
use crate::config::Config;
use crate::fs::to_child_str_path;
use crate::loader::load_data;
use crate::metadata::{Author, EntryKey as Key, Entry, Metadata};

fn merge_authors<'a>(meta: &'a mut Value, c: &Config) -> &'a mut Value {
    *meta.pointer_mut("/website/metadata/authors").unwrap() =
        json!(c.website.metadata.as_ref().map_or(
            // TODO: handle email in format like: "name <email>"
            c.authors.as_ref().map_or(vec![], |a| {
                a.iter().map(|n| Author::new(n)).collect()
            }),
            |m| {
                m.authors
                    .as_ref()
                    .map_or(vec![], |v| v.iter().map(Author::from).collect())
            }
        ));
    meta
}

/// generates an entry into file in the dst directory and returns its metadata.
pub fn make_entry(
    buf: &Path,
    dst: &Path,
    reg: &Handlebars,
    cfg: &Config,
) -> Result<Entry, Error> {
    let mut e = load_data(&fs::read_to_string(buf)?);
    if !e.has(Key::Content) {
        let empty = Entry::new();
        return Ok(empty);
    }

    // assign _path (a part of url) and slug
    let name = if e.has(Key::Slug) {
        e.get(Key::Slug).unwrap()
    } else {
        let stem = buf.file_stem().unwrap().to_string_lossy().into_owned();
        stem + ".html"
    };
    let path = dst.join(&name);

    e.add(Key::_Path, to_child_str_path(&path));
    e.add(Key::Slug, name);

    let mut meta = &mut json!({
        "website": cfg.website.to_json(),
        "article": e.to_json(),
    });
    meta = merge_authors(meta, cfg);

    let result = reg
        .render("layout", meta)
        .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;

    e.add(Key::_Body, result);
    Ok(e)
}

/// copies asset files to dst directory.
pub fn copy_assets(theme: &str, dir: &Path) -> Result<(), Error> {
    #[allow(clippy::vec_init_then_push)]
    for (n, s) in match theme {
        "documentation" => {
            include_static_file!("documentation", "css/index.css", "robots.txt")
        }
        _ => include_static_file!("blog", "css/index.css", "robots.txt"),
    } {
        let dst = dir.join(n);
        write_entry(&s, &dst)?;
    }
    Ok(())
}

/// writes the index file into dst directory.
pub fn write_index(
    dat: &mut Vec<Entry>,
    reg: &Handlebars,
    cfg: &Config,
) -> Result<(), Error> {
    let dst_dir = cfg.build.get_target_dir();
    let path = Path::new(&dst_dir).join("index.html");
    let mut file = fs::File::create(path)?;

    // reverse sorting
    dat.sort_by(|a, b| b.cmp(a));

    let mut meta = &mut json!({
        "website": cfg.website.to_json(),
        "headlines": json!(&dat),
    });
    meta = merge_authors(meta, cfg);

    let result = reg
        .render("layout", meta)
        .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
    file.write_all(result.as_bytes())?;
    Ok(())
}

/// creates a file.
pub fn write_entry(s: &str, dst: &Path) -> Result<(), Error> {
    fs::write(dst, s)?;
    Ok(())
}
