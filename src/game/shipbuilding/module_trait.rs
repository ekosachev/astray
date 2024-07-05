use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct ModuleTrait {
    name: String,
}