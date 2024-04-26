use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Getters)]
pub struct ResearchField {
    name: String,
    id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Getters)]
pub struct Research {
    name: String,
    id: String,
    cost: u32,
    field: String,
    required_any: Vec<String>,
    required_all: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Getters)]
pub struct ResearchProgress {
    id: String,
    progress: u32,
    speed: u32,
    is_finished: bool,
}

impl ResearchField {
    pub fn load_from_file(filepath: &str) -> Vec<Self> {
        serde_json::from_str(&std::fs::read_to_string(filepath).unwrap()).unwrap()
    }
}

impl From<ResearchField> for String {
    fn from(value: ResearchField) -> Self {
        value.name
    }
}

impl Research {
    pub fn load_from_file(filepath: &str) -> Vec<Self> {
        serde_json::from_str(&std::fs::read_to_string(filepath).unwrap()).unwrap()
    }
}

impl ResearchProgress {
    pub fn load_from_file(filepath: &str) -> Vec<Self> {
        serde_json::from_str(&std::fs::read_to_string(filepath).unwrap()).unwrap()
    }
}