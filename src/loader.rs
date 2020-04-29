//!
use crate::metadata::{Key, Metadata, META_PREFIX, META_SUFFIX};
use crate::renderer::render;

/// read metadata at the beginnig of each file and its content.
pub fn load_data(s: &str) -> Metadata {
    let mut v = Metadata::default();

    // TODO: global
    v.add(Key::Name, "Name".to_string());
    v.add(Key::Url, "/".to_string());

    // article
    v.add(Key::Slug, "".to_string());

    v.add(Key::Date, "".to_string());
    v.add(Key::Lang, "en".to_string());
    v.add(Key::Title, "".to_string());
    v.add(Key::Description, "".to_string());

    let mut iter = s.lines();
    loop {
        match iter.next() {
            Some(a) if a.starts_with(META_PREFIX) => {
                let p: Vec<&str> = a.splitn(2, META_SUFFIX).collect();
                if p.len() == 2 {
                    let key_value = p[0].replace(META_PREFIX, "");
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

    // data
    // v.add(Key::Content, "".to_string());

    let body: Vec<&str> = iter.collect();
    if let Ok(c) = render(&format!("{}\n", body.join("\n"))) {
        v.add(Key::from(&("content".to_string())), c);
    }
    v
}
