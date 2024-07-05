use ratatui::prelude::Color;
use serde::de::DeserializeOwned;

use crate::game::celestial_bodies::Displayable;
use crate::game::shipbuilding::module_trait::ModuleTrait;

pub enum ShipModuleType {
    SublightThruster,
}

impl Displayable for ShipModuleType {
    fn get_name(&self) -> String {
        match self {
            ShipModuleType::SublightThruster => { "Sublight Thruster" }
        }.to_string()
    }

    fn get_menu_color(&self) -> Color {
        match self {
            ShipModuleType::SublightThruster => { Color::Indexed(75) }
        }
    }
}

impl From<String> for ShipModuleType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Sublight Thruster" => { Self::SublightThruster }
            _ => panic!()
        }
    }
}

pub trait ShipModule {
    fn get_traits(&self) -> Vec<ModuleTrait>;

    fn load_from_file<T>(filepath: &str) -> Vec<T>
    where
        T: ShipModule + DeserializeOwned,
    {
        let file_contents = std::fs::read_to_string(filepath).unwrap();

        serde_json::from_str(&file_contents).unwrap()
    }
}
