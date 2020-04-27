use std::cmp::{Eq, Ord, Ordering, PartialEq};
use std::fmt;
use std::hash::{Hash, Hasher};

use serde::{Serialize, Serializer};

pub enum Key {
    // site
    Name,

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
            Self::Content => write!(f, "content"),
            Self::Date => write!(f, "date"),
            Self::Description => write!(f, "description"),
            Self::Lang => write!(f, "lang"),
            Self::Name => write!(f, "name"),
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
            "name" => Self::Name,
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
