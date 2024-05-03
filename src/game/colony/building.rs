use ratatui::prelude::Color;
use serde::{Deserialize, Serialize};

use crate::game::celestial_bodies::Displayable;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Hash)]
pub enum FactoryType {
    // Primary resources to secondary resources
    ElectronicsFactory,
    KeroseneFactory,
    HeatResistantAlloyFactory,
    SuperconductorsFactory,
    PlasticsFactory,
    CompositesFactory,
    RadioactivePelletsFactory,

    // Secondary resources to components
    EngineNozzlesFactory,
    MicroprocessorsFactory,
    SensorsFactory,
    FuelRodsFactory,
}

impl Into<String> for FactoryType {
    fn into(self) -> String {
        match self {
            FactoryType::ElectronicsFactory => { "Electronics factory".to_string() }
            FactoryType::KeroseneFactory => { "Kerosene factory".to_string() }
            FactoryType::HeatResistantAlloyFactory => { "Heat resistant alloy factory".to_string() }
            FactoryType::SuperconductorsFactory => { "Superconductors factory".to_string() }
            FactoryType::PlasticsFactory => { "Plastics factory".to_string() }
            FactoryType::CompositesFactory => { "Composites factory".to_string() }
            FactoryType::RadioactivePelletsFactory => { "Radioactive pellets factory".to_string() }
            FactoryType::EngineNozzlesFactory => { "Engine nozzles factory".to_string() }
            FactoryType::MicroprocessorsFactory => { "Microprocessors factory".to_string() }
            FactoryType::SensorsFactory => { "Sensors factory".to_string() }
            FactoryType::FuelRodsFactory => { "Fuel rods factory".to_string() }
        }
    }
}

impl FactoryType {
    pub fn get_construction_time(&self) -> u32 {
        match self {
            FactoryType::ElectronicsFactory => { 75 }
            FactoryType::KeroseneFactory => { 75 }
            FactoryType::HeatResistantAlloyFactory => { 75 }
            FactoryType::SuperconductorsFactory => { 75 }
            FactoryType::PlasticsFactory => { 75 }
            FactoryType::CompositesFactory => { 100 }
            FactoryType::RadioactivePelletsFactory => { 75 }
            FactoryType::EngineNozzlesFactory => { 130 }
            FactoryType::MicroprocessorsFactory => { 130 }
            FactoryType::SensorsFactory => { 130 }
            FactoryType::FuelRodsFactory => { 130 }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Hash)]
pub enum BuildingType {
    Mine,
    Factory(FactoryType),
    Spaceport,
    DryDock,
}

impl Into<Color> for BuildingType {
    fn into(self) -> Color {
        match self {
            BuildingType::Mine => Color::Gray,
            BuildingType::Factory(_) => Color::LightRed,
            BuildingType::Spaceport => Color::LightCyan,
            BuildingType::DryDock => Color::LightMagenta,
        }
    }
}

impl Into<String> for BuildingType {
    fn into(self) -> String {
        match self {
            BuildingType::Mine => { "Mine".to_string() }
            BuildingType::Factory(factory_type) => { factory_type.into() }
            BuildingType::Spaceport => { "Spaceport".to_string() }
            BuildingType::DryDock => { "Dry dock".to_string() }
        }
    }
}

impl Displayable for BuildingType {
    fn get_properties(&self) -> Vec<Vec<String>> {
        Vec::new()
    }
    fn get_name(&self) -> String {
        self.clone().into()
    }

    fn get_menu_color(&self) -> Color {
        self.clone().into()
    }
}

impl BuildingType {
    pub fn get_construction_time(&self) -> u32 {
        match self {
            BuildingType::Mine => { 100 }
            BuildingType::Factory(ft) => { ft.get_construction_time() }
            BuildingType::Spaceport => { 150 }
            BuildingType::DryDock => { 130 }
        }
    }

    pub fn is_producing_resources(&self) -> bool {
        match self {
            BuildingType::Mine => { false }
            BuildingType::Factory(_) => { true }
            BuildingType::Spaceport => { false }
            BuildingType::DryDock => { false }
        }
    }
}

