use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum StringOrMap {
    String(String),
    HashMap(HashMap<String, String>),
}

impl Default for StringOrMap {
    fn default() -> Self {
        StringOrMap::String(String::new())
    }
}
