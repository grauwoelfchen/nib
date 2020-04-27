use std::fs;
use std::io::{BufWriter, Error, ErrorKind, Write};
use std::path::{Path, PathBuf};

use glob::glob;
use handlebars::{Handlebars, no_escape};
use rst_parser::parse;
use rst_renderer::render_html;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

mod key;
mod var;

use crate::key::Key;
pub use crate::var::Variables;

const META_KEY: &str = ".. ";

/// loads variables from metadata at the beginnig and file content.
fn load(s: &str) -> Variables {
    let mut v = Variables::default();

    // TODO
    v.add(Key::Name, "Name".to_string());

    // default
    v.add(Key::Date, "".to_string());
    v.add(Key::Lang, "en".to_string());
    v.add(Key::Title, "".to_string());
    v.add(Key::Description, "".to_string());

    // optional
    // v.add(Key::Content, "".to_string());

    let mut iter = s.lines();
    loop {
        match iter.next() {
            Some(a) if a.starts_with(META_KEY) => {
                let p: Vec<&str> = a.splitn(2, "::").collect();
                if p.len() == 2 {
                    let key_value = p[0].replace(META_KEY, "");
                    let key = Key::from(&key_value);
                    if key == Key::Unknown {
                        eprintln!("Unknown key: {}", &key_value);
                    } else {
                        v.add(key, p[1].trim_start().to_owned());
                    }
                }
            },
            _ => break,
        }
    }

    let body: Vec<&str> = iter.collect();
    if let Ok(c) = render(&format!("{}\n", body.join("\n"))) {
        v.add(Key::from(&("content".to_string())), c);
    }
    v
}

/// renders HTML result in partial mode.
fn render(s: &str) -> Result<String, Error> {
    match parse(s) {
        Err(e) => {
            eprintln!("err: {}", e);
            Err(Error::new(ErrorKind::InvalidInput, e))
        },
        Ok(doc) => {
            let buf = Vec::new();
            let mut stream = BufWriter::new(buf);
            let standalone = false;

            render_html(&doc, &mut stream, standalone)
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
            let r = stream.into_inner().unwrap();
            Ok(String::from_utf8_lossy(&r).into_owned())
        },
    }
}

pub fn load_registry<'a>() -> Handlebars<'a> {
    let mut reg = Handlebars::new();
    let _ = reg.register_template_file("_article", "tmpl/_article.hbs");
    let _ = reg.register_template_file("_footer", "tmpl/_footer.hbs");
    let _ = reg.register_template_file("_header", "tmpl/_header.hbs");
    let _ = reg.register_template_file("_headline", "tmpl/_headline.hbs");
    let _ = reg.register_template_file("_index", "tmpl/_index.hbs");
    let _ = reg.register_template_file("_layout", "tmpl/_layout.hbs");
    let _ = reg.register_template_file("_layout.idx", "tmpl/_layout.index.hbs");
    reg
}

pub fn add_escape_fn(reg: &mut Handlebars) {
    reg.register_escape_fn(no_escape)
}

pub fn rem_escape_fn(reg: &mut Handlebars) {
    reg.unregister_escape_fn()
}

/// generates a HTML file into dst directory and returns variables.
pub fn generate_entry(
    e: &PathBuf,
    reg: &Handlebars,
    dst: &str,
) -> Result<Variables, Error>
{
    let mut data = load(&fs::read_to_string(e)?);
    if !data.has(Key::Content) {
        let empty = Variables::new();
        return Ok(empty);
    }

    let stem = e.file_stem().unwrap().to_string_lossy().into_owned();
    let name = stem + ".html";
    let path = Path::new(dst).join(&name);

    data.add(Key::Slug, name);
    let mut file = fs::File::create(path)?;

    let result = reg
        .render("_layout", &data.to_json())
        .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
    file.write_all(result.as_bytes())?;
    Ok(data)
}

pub fn generate_index(
    dat: &mut Vec<Variables>,
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
    for v in dat {
        result = result +
            &reg.render("_headline", &v.to_json())
                .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
    }

    result = reg
        .render(
            "_index",
            &json!({
                "title": title,
                "content": "<ul>".to_string() + &result + "</ul>",
            }),
        )
        .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
    result = reg
        .render(
            "_layout.idx",
            &json!({
                "lang": lang,
                "title": title,
                "content": &result,
            }),
        )
        .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;

    file.write_all(result.as_bytes())?;
    Ok(())
}

/// gets file entries.
pub fn get_entries(path: &str) -> glob::Paths {
    glob(path).expect("failed to read glob pattern")
}
