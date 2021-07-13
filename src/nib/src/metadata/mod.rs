mod author;
mod entry;
mod heading;

use std::fmt;
use std::hash::Hash;

use serde_json::Value;

pub use author::{AuthorKey, Author};
pub use entry::{EntryKey, Entry, KEY_PREFIX, KEY_SUFFIX};
pub use heading::Heading;

use serde::Serialize;

pub trait Metadata<T>
where
    T: Serialize + fmt::Display + Hash,
{
    fn new() -> Self;
    fn add(&mut self, key: T, value: String) -> Option<String>;
    fn get(&self, key: T) -> Option<String>;
    fn has(&self, key: T) -> bool;
    fn to_json(&self) -> Value;
}
