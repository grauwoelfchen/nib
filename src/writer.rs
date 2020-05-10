//!
use std::fs;
use std::io::{Error, ErrorKind, Write};
use std::path::{Path, PathBuf};

use handlebars::Handlebars;
use serde_json::Value;

use crate::config::Config;
use crate::loader::load_data;
use crate::metadata::{Author, EntryKey as Key, Entry, Metadata};

/// copies file under dst directory.
pub fn move_entry(buf: &PathBuf, dst: &Path) -> Result<(), Error> {
    let to = dst.join(buf.file_name().unwrap());
    fs::copy(buf.to_str().unwrap(), to)?;
    Ok(())
}

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

/// puts an entry into file in the dst directory and returns its metadata.
pub fn save_entry(
    buf: &PathBuf,
    reg: &Handlebars,
    c: &Config,
) -> Result<Entry, Error>
{
    let mut e = load_data(&fs::read_to_string(buf)?);
    if !e.has(Key::Content) {
        let empty = Entry::new();
        return Ok(empty);
    }

    let stem = buf.file_stem().unwrap().to_string_lossy().into_owned();
    let name = stem + ".html";
    let path = Path::new(&c.build.get_target_dir()).join(&name);

    e.add(Key::Slug, name);
    let mut file = fs::File::create(path)?;

    let mut meta = &mut json!({
        "website": c.website.to_json(),
        "article": e.to_json(),
    });
    meta = merge_authors(meta, c);

    let result = reg
        .render("layout", meta)
        .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
    file.write_all(result.as_bytes())?;
    Ok(e)
}

/// creates a index file into dst directory.
pub fn make_index(
    dat: &mut Vec<Entry>,
    reg: &Handlebars,
    c: &Config,
) -> Result<(), Error>
{
    let dst_dir = c.build.get_target_dir();
    let path = Path::new(&dst_dir).join("index.html");
    let mut file = fs::File::create(path)?;

    let mut meta = &mut json!({
        "website": c.website.to_json(),
        "content": "",
    });
    meta = merge_authors(meta, c);

    let mut result: String = "".to_string();
    for d in dat {
        result = result +
            &reg.render("headline", &json!({"article": &d.to_json()}))
                .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
    }

    *meta.pointer_mut("/content").unwrap() = json!(
        "<div class=\"content\"><ul>".to_string() + &result + "</ul></div>"
    );
    result = reg
        .render("layout", meta)
        .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;

    file.write_all(result.as_bytes())?;
    Ok(())
}
