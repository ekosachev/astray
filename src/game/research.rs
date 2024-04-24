use serde::Deserialize;

#[derive(Deserialize)]
pub struct Research {
    name: String,
    id: String,
    cost: u32,
    required_any: Vec<String>,
    required_all: Vec<String>,
}

impl Research {
    pub fn load_from_file(filepath: &str) -> Vec<Self> {
        let deserialized: Vec<Self> = serde_json::from_str(&std::fs::read_to_string(filepath).unwrap()).unwrap();

        deserialized
    }
}