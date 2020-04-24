use std::collections::HashMap;

use crate::key::Key;

#[derive(Serialize, Default)]
pub struct Variables {
    pub map: HashMap<Key, String>,
}

impl Variables {
    pub fn add(&mut self, key: Key, value: String) -> Option<String> {
        self.map.insert(key, value)
    }

    pub fn has(&self, key: &Key) -> bool {
        self.map.get(key).is_some()
    }
}
