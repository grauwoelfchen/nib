//!
use std::fs;
use std::io::{Error, ErrorKind, Write};
use std::path::Path;

use handlebars::Handlebars;
use rst_parser::parse_only;
use document_tree::{Document, HasChildren};
use document_tree::element_categories::{
    BodyElement as BE, StructuralSubElement as SSE, SubStructure as SS,
    TextOrInlineElement as TOIE,
};
use serde_json::Value;
use regex::Regex;

use crate::include_static_file;
use crate::config::Config;
use crate::document::MyDocument;
use crate::fs::to_child_str_path;
use crate::loader::load_data;
use crate::metadata::{Author, EntryKey as Key, Entry, Heading, Metadata};

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

fn extract_heading_tree(doc: &mut Document) -> Vec<Heading> {
    let md: &mut MyDocument =
        unsafe { &mut *(doc as *mut Document as *mut MyDocument) };

    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^<(h[1-6])>(.*)</h[1-6]>$").unwrap();
    }

    let mut tree: Vec<Heading> = vec![];
    for (_, e) in md.children().iter().enumerate() {
        if let SSE::SubStructure(ref s1) = e {
            if let SS::BodyElement(ref be) = **s1 {
                if let BE::Paragraph(ref p) = **be {
                    for c in p.children() {
                        if let TOIE::String(ref s) = c {
                            if let Some(cap) = RE.captures(&s) {
                                let h = Heading::new(&cap[1], &cap[2]);
                                tree.push(h);
                            }
                        }
                    }
                }
            }
        }
    }
    tree
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

    // TOC
    let s = e.get(Key::Content).unwrap();
    let mut tree: Vec<Heading> = vec![];
    if let Ok(mut doc) = parse_only(&s) {
        tree = extract_heading_tree(&mut doc);
    }

    let mut meta = &mut json!({
        "website": cfg.website.to_json(),
        "article": e.to_json(),
        "tree": tree,
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
