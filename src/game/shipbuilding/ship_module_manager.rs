use crate::game::shipbuilding::ship_module::{ShipModule, ShipModuleType};
use crate::game::shipbuilding::sublight_engine::SublightEngine;

pub struct ShipModuleManager {
    sublight_engines: Vec<SublightEngine>,
}

const SHIP_MODULES_PATH: &str = "./assets/ship_modules/";

impl ShipModuleManager {
    pub fn new() -> Self {
        Self {
            sublight_engines: SublightEngine::load_from_file(
                (SHIP_MODULES_PATH.to_string() + "sublight_engines.json5").as_str()
            ),
        }
    }

    pub fn get_ship_module_types(&self) -> Vec<ShipModuleType> {
        vec![
            ShipModuleType::SublightThruster,
        ]
    }

    pub fn get_ship_module_type_by_name(&self, name: String) -> ShipModuleType {
        ShipModuleType::from(name)
    }

    pub fn get_ship_modules_by_type<T: ShipModule>(&self, module_type: ShipModuleType) -> Vec<T> {
        let modules = match module_type {
            ShipModuleType::SublightThruster => { self.sublight_engines.clone() }
        };
        
        modules
    }
}