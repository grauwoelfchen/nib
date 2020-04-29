use std::collections::HashMap;
use std::cmp::{Eq, Ord, Ordering, PartialEq};
use std::fmt;
use std::hash::{Hash, Hasher};

use serde::{Serialize, Serializer};
use serde_json::Value;

pub const META_PREFIX: &str = ".. ";
pub const META_SUFFIX: &str = "::";

pub enum Key {
    // site
    Name,
    Url,

    // auto
    Slug,

    // article
    Date,
    Lang,
    Title,
    Description,
    Content,

    Unknown,
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
            Self::Name => write!(f, "name"),
            Self::Url => write!(f, "url"),

            Self::Slug => write!(f, "slug"),

            Self::Date => write!(f, "date"),
            Self::Description => write!(f, "description"),
            Self::Lang => write!(f, "lang"),
            Self::Title => write!(f, "title"),

            Self::Content => write!(f, "content"),

            _ => write!(f, "unknown"),
        }
    }
}

impl From<&String> for Key {
    fn from(s: &String) -> Self {
        match s.to_ascii_lowercase().as_ref() {
            "name" => Self::Name,
            "url" => Self::Url,

            "slug" => Self::Slug,

            "date" => Self::Date,
            "description" => Self::Description,
            "lang" => Self::Lang,
            "title" => Self::Title,

            "content" => Self::Content,

            _ => Self::Unknown,
        }
    }
}

impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}

#[derive(Serialize)]
pub struct Metadata {
    pub map: HashMap<Key, String>,
}

impl Default for Metadata {
    fn default() -> Self {
        let map: HashMap<Key, String> = HashMap::new();
        Metadata { map }
    }
}

impl Metadata {
    pub fn new() -> Self {
        Metadata::default()
    }

    pub fn add(&mut self, key: Key, value: String) -> Option<String> {
        self.map.insert(key, value)
    }

    pub fn get(&self, key: Key) -> Option<String> {
        self.map.get(&key).map(|v| v.to_owned())
    }

    pub fn has(&self, key: Key) -> bool {
        self.map.get(&key).is_some()
    }

    pub fn to_json(&self) -> Value {
        json!(self.map)
    }
}
