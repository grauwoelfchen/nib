use crate::key::Key;
use crate::var::Variables;
use crate::renderer::render;

const META_KEY: &str = ".. ";

/// loads variables from metadata at the beginnig and file content.
pub fn load(s: &str) -> Variables {
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
