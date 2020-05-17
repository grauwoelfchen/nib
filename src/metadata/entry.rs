use std::collections::{BTreeMap, HashMap};
use std::cmp::{Eq, Ord, Ordering, PartialEq};
use std::fmt;
use std::hash::{Hash, Hasher};

use serde::{Serialize, Serializer};
use serde_json::Value;

use crate::metadata::Metadata;

pub const KEY_PREFIX: &str = ".. ";
pub const KEY_SUFFIX: &str = "::";

pub enum EntryKey {
    Content,
    Date,
    Description,
    Lang,
    Slug,
    Title,
    Unknown,

    // auto derive
    _Path,
}

impl Serialize for EntryKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

impl PartialEq for EntryKey {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Eq for EntryKey {}

impl PartialOrd for EntryKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EntryKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_string().cmp(&other.to_string())
    }
}

impl fmt::Display for EntryKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Content => write!(f, "content"),
            Self::Date => write!(f, "date"),
            Self::Description => write!(f, "description"),
            Self::Lang => write!(f, "lang"),
            Self::Slug => write!(f, "slug"),
            Self::Title => write!(f, "title"),
            Self::_Path => write!(f, "_path"),
            _ => write!(f, "unknown"),
        }
    }
}

impl From<&String> for EntryKey {
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

impl Hash for EntryKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}

pub struct Entry {
    _map: HashMap<EntryKey, String>,
}

impl Serialize for Entry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let map: BTreeMap<_, _> = self._map.iter().collect();
        map.serialize(serializer)
    }
}

impl Default for Entry {
    fn default() -> Self {
        let map: HashMap<EntryKey, String> = HashMap::new();
        Entry { _map: map }
    }
}

impl Metadata<EntryKey> for Entry {
    fn new() -> Self {
        Self::default()
    }

    fn add(&mut self, key: EntryKey, value: String) -> Option<String> {
        self._map.insert(key, value)
    }

    fn get(&self, key: EntryKey) -> Option<String> {
        self._map.get(&key).map(|v| v.to_owned())
    }

    fn has(&self, key: EntryKey) -> bool {
        let v = self._map.get(&key);
        v.is_some() && !v.unwrap().is_empty()
    }

    fn to_json(&self) -> Value {
        json!(self._map)
    }
}