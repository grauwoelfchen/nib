use std::collections::{BTreeMap, HashMap};
use std::cmp::{Eq, Ord, Ordering, PartialEq};
use std::fmt;
use std::hash::{Hash, Hasher};

use serde::{Serialize, Serializer};
use serde_json::Value;

use crate::config;
use crate::metadata::Metadata;

pub enum AuthorKey {
    Avatar,
    Bio,
    Email,
    Name,
    Nick,
    Unknown,
}

impl Serialize for AuthorKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl PartialEq for AuthorKey {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Eq for AuthorKey {}

impl PartialOrd for AuthorKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AuthorKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_string().cmp(&other.to_string())
    }
}

impl fmt::Display for AuthorKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Avatar => write!(f, "avatar"),
            Self::Bio => write!(f, "bio"),
            Self::Email => write!(f, "email"),
            Self::Name => write!(f, "name"),
            Self::Nick => write!(f, "nick"),
            _ => write!(f, "unknown"),
        }
    }
}

impl From<&String> for AuthorKey {
    fn from(s: &String) -> Self {
        match s.to_ascii_lowercase().as_ref() {
            "avatar" => Self::Avatar,
            "bio" => Self::Bio,
            "email" => Self::Email,
            "name" => Self::Name,
            "nick" => Self::Nick,
            _ => Self::Unknown,
        }
    }
}

impl Hash for AuthorKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}

pub struct Author {
    _map: HashMap<AuthorKey, String>,
}

impl Serialize for Author {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let map: BTreeMap<_, _> = self._map.iter().collect();
        map.serialize(serializer)
    }
}

impl Default for Author {
    fn default() -> Self {
        let map: HashMap<AuthorKey, String> = HashMap::new();
        Author { _map: map }
    }
}

impl From<&config::Author> for Author {
    fn from(a: &config::Author) -> Self {
        let mut author = Self::default();
        author.add(
            AuthorKey::Avatar,
            a.avatar.clone().map_or("".to_string(), |s| s),
        );
        author.add(AuthorKey::Bio, a.bio.clone().map_or("".to_string(), |s| s));
        author.add(
            AuthorKey::Email,
            a.email.clone().map_or("".to_string(), |s| s),
        );
        author.add(AuthorKey::Name, a.name.to_string());
        author.add(
            AuthorKey::Nick,
            a.nick.clone().map_or("".to_string(), |s| s),
        );
        author
    }
}

impl Metadata<AuthorKey> for Author {
    fn new() -> Self {
        Self::default()
    }

    fn add(&mut self, key: AuthorKey, value: String) -> Option<String> {
        self._map.insert(key, value)
    }

    fn get(&self, key: AuthorKey) -> Option<String> {
        self._map.get(&key).map(|v| v.to_owned())
    }

    fn has(&self, key: AuthorKey) -> bool {
        self._map.get(&key).is_some()
    }

    fn to_json(&self) -> Value {
        json!(self._map)
    }
}

impl Author {
    pub fn new(name: &str) -> Self {
        let mut author = Self::default();
        author.add(AuthorKey::Name, name.to_string());
        author
    }
}
