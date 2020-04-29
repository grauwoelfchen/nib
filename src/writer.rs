//!
use std::fs;
use std::io::{Error, ErrorKind, Write};
use std::path::{Path, PathBuf};

use handlebars::Handlebars;

use crate::loader::load_data;
use crate::metadata::{Key, Metadata};

/// copies file under dst directory.
pub fn move_entry(e: &PathBuf, dst: &Path) -> Result<(), Error> {
    let to = dst.join(e.file_name().unwrap());
    fs::copy(e.to_str().unwrap(), to)?;
    Ok(())
}

/// puts an entry into file in the dst directory and returns its metadata.
pub fn save_entry(
    e: &PathBuf,
    reg: &Handlebars,
    dst: &str,
) -> Result<Metadata, Error>
{
    let mut data = load_data(&fs::read_to_string(e)?);
    if !data.has(Key::Content) {
        let empty = Metadata::new();
        return Ok(empty);
    }

    let stem = e.file_stem().unwrap().to_string_lossy().into_owned();
    let name = stem + ".html";
    let path = Path::new(dst).join(&name);

    data.add(Key::Slug, name);
    let mut file = fs::File::create(path)?;

    let meta = &mut data.to_json();
    *meta.pointer_mut("/url").unwrap() = json!("/");

    let result = reg
        .render("layout", meta)
        .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
    file.write_all(result.as_bytes())?;
    Ok(data)
}

/// creates a index file into dst directory.
pub fn make_index(
    dat: &mut Vec<Metadata>,
    reg: &Handlebars,
    dst: &str,
) -> Result<(), Error>
{
    let dst_dir = Path::new(dst);
    let path = dst_dir.join("index.html");
    let mut file = fs::File::create(path)?;

    // TODO
    let lang = dat[0].get(Key::Lang).ok_or(ErrorKind::InvalidInput)?;
    let title = dat[0].get(Key::Name).ok_or(ErrorKind::InvalidInput)?;

    let mut result: String = "".to_string();
    for d in dat {
        result = result +
            &reg.render("headline", &d.to_json())
                .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
    }

    result = reg
        .render(
            "index",
            &json!({
                "title": title,
                "content": "<ul>".to_string() + &result + "</ul>",
            }),
        )
        .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
    result = reg
        .render(
            "layout.idx",
            &json!({
                "url": "/",
                "lang": lang,
                "title": title,
                "content": &result,
            }),
        )
        .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;

    file.write_all(result.as_bytes())?;
    Ok(())
}
