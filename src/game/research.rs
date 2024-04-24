use std::char::decode_utf16;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct ResearchField {
    name: String,
    id: String,
}

#[derive(Deserialize, Clone)]
pub struct Research {
    name: String,
    id: String,
    cost: u32,
    field: String,
    required_any: Vec<String>,
    required_all: Vec<String>,
}

impl ResearchField {
    pub fn load_from_file(filepath: &str) -> Vec<Self> {
        serde_json::from_str(&std::fs::read_to_string(filepath).unwrap()).unwrap()
    }
}

impl Research {
    pub fn load_from_file(filepath: &str) -> Vec<Self> {
        serde_json::from_str(&std::fs::read_to_string(filepath).unwrap()).unwrap()
    }
}