use std::collections::HashMap;

use serde_json::Value;

use crate::key::Key;

#[derive(Serialize)]
pub struct Variables {
    pub map: HashMap<Key, String>,
}

impl Default for Variables {
    fn default() -> Self {
        let map: HashMap<Key, String> = HashMap::new();
        Variables { map }
    }
}

impl Variables {
    pub fn new() -> Self {
        Variables::default()
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
