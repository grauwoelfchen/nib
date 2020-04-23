use std::collections::HashMap;
use std::cmp::{Eq, Ord, Ordering, PartialEq};
use std::fmt;
use std::fs;
use std::hash::{Hash, Hasher};
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

use serde::{Serialize, Serializer};

const META_KEY: &str = ".. ";

enum Key {
    Content,
    Date,
    Lang,
    Slug,
    Title,
    Description,
    Unknown,
}

#[derive(Serialize, Default)]
struct Variables {
    map: HashMap<Key, String>,
}

impl Variables {
    fn add(&mut self, key: Key, value: String) -> Option<String> {
        self.map.insert(key, value)
    }

    fn has(&self, key: &Key) -> bool {
        self.map.get(key).is_some()
    }
}

impl Serialize for Key {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Eq for Key {}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Key {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_string().cmp(&other.to_string())
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Content => write!(f, "content"),
            Self::Date => write!(f, "date"),
            Self::Description => write!(f, "description"),
            Self::Lang => write!(f, "lang"),
            Self::Slug => write!(f, "slug"),
            Self::Title => write!(f, "title"),
            _ => write!(f, "unknown"),
        }
    }
}

impl From<&String> for Key {
    fn from(s: &String) -> Self {
        match s.to_ascii_lowercase().as_ref() {
            "content" => Self::Content,
            "date" => Self::Date,
            "description" => Self::Description,
            "lang" => Self::Lang,
            "slug" => Self::Slug,
            "title" => Self::Title,
            _ => Self::Unknown,
        }
    }
}

impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}

/// loads variables from metadata at the beginnig and file content.
fn load(s: &str) -> Variables {
    let mut v = Variables::default();

    // default
    v.add(Key::Date, "".to_string());
    v.add(Key::Lang, "en".to_string());
    v.add(Key::Slug, "/".to_string());
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
    let _ = reg.register_template_file("_layout", "tmpl/_layout.hbs");
    reg
}

/// generates a HTML file into dst directory.
pub fn generate_entry(
    e: &PathBuf,
    reg: &mut Handlebars,
    dst: &str,
) -> Result<(), Error>
{
    let stem = e.file_stem().unwrap().to_string_lossy().into_owned();
    let name = vec![stem, "html".to_string()].join(".");

    let data = load(&fs::read_to_string(e)?);
    if !data.has(&Key::Content) {
        return Ok(());
    }

    let path = Path::new(dst).join(name);
    let mut file = fs::File::create(path)?;

    reg.register_escape_fn(no_escape);
    let result = reg
        .render("_layout", &json!(&data.map))
        .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
    file.write_all(result.as_bytes())?;
    Ok(())
}

/// gets file entries.
pub fn get_entries(path: &str) -> glob::Paths {
    glob(path).expect("failed to read glob pattern")
}
