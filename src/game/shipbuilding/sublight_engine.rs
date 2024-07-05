use serde::Deserialize;

use crate::game::shipbuilding::module_trait::ModuleTrait;
use crate::game::shipbuilding::ship_module::ShipModule;

#[derive(Clone, Deserialize)]
pub struct SublightEngine {
    name: String,
    traits: Vec<ModuleTrait>,
    is_unlocked: bool,
}

impl ShipModule for SublightEngine {
    fn get_traits(&self) -> Vec<ModuleTrait> {
        self.traits.clone()
    }
}