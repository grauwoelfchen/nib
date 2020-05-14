//!
use crate::metadata::{EntryKey as Key, Entry, KEY_PREFIX, KEY_SUFFIX, Metadata};
use crate::renderer::render;

/// read entry metadata at the beginnig of each file and its content.
pub fn load_data(s: &str) -> Entry {
    let mut v = Entry::default();

    v.add(Key::Date, "".to_string());
    v.add(Key::Description, "".to_string());
    v.add(Key::Lang, "en".to_string());
    v.add(Key::Title, "".to_string());

    let mut iter = s.lines();
    loop {
        match iter.next() {
            Some(a) if a.starts_with(KEY_PREFIX) => {
                let p: Vec<&str> = a.splitn(2, KEY_SUFFIX).collect();
                if p.len() == 2 {
                    let key_value = p[0].replace(KEY_PREFIX, "");
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
