use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchField {
    name: String,
    id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Research {
    name: String,
    id: String,
    cost: u32,
    field: String,
    required_any: Vec<String>,
    required_all: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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