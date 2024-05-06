use std::collections::HashMap;

use log::info;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};

use crate::game::celestial_bodies::Displayable;
use crate::game::colony::building::{BuildingType, FactoryType};
use crate::game::colony::construction_process::ConstructionProcess;
use crate::game::resource::resource::{ResourceDeposit, ResourceTransaction};
use crate::game::resource::resource_manager::ResourceManager;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct BuildingManager {
    buildings: HashMap<BuildingType, u32>,
    construction: Vec<ConstructionProcess>,
    construction_limit: u32,
}

impl Default for BuildingManager {
    fn default() -> Self {
        Self {
            buildings: HashMap::from([
                (BuildingType::DryDock, 0),
                (BuildingType::Mine, 0),
                (BuildingType::Spaceport, 0),
                (BuildingType::Factory(FactoryType::ElectronicsFactory), 0),
                (BuildingType::Factory(FactoryType::KeroseneFactory), 0),
                (BuildingType::Factory(FactoryType::HeatResistantAlloyFactory), 0),
                (BuildingType::Factory(FactoryType::SuperconductorsFactory), 0),
                (BuildingType::Factory(FactoryType::PlasticsFactory), 0),
                (BuildingType::Factory(FactoryType::CompositesFactory), 0),
                (BuildingType::Factory(FactoryType::RadioactivePelletsFactory), 0),
                (BuildingType::Factory(FactoryType::EngineNozzlesFactory), 0),
                (BuildingType::Factory(FactoryType::MicroprocessorsFactory), 0),
                (BuildingType::Factory(FactoryType::SensorsFactory), 0),
                (BuildingType::Factory(FactoryType::FuelRodsFactory), 0),
            ]),
            construction: Vec::new(),
            construction_limit: 10,
        }
    }
}

impl BuildingManager {
    pub fn new() -> Self {
        Self::default()
    }

    fn add_building(&mut self, building_type: &BuildingType) {
        let current_amount = *self.buildings.get(
            building_type
        ).unwrap_or(&0);

        self.buildings.insert(
            building_type.clone(),
            current_amount + 1,
        );
    }

    pub fn start_construction(
        &mut self,
        building_type: BuildingType,
    ) {
        if self.construction.len() < (self.construction_limit as usize) {
            self.construction.push(
                ConstructionProcess::from(building_type)
            )
        }
        info!("Started construction");
    }

    pub fn update_construction(&mut self) {
        if !self.construction.is_empty() {
            let is_finished = self.construction[0].update(1);
            if is_finished {
                let building = self.construction[0].building_type();
                *self.buildings.get_mut(building).unwrap() += 1;
                self.construction.remove(0);
            }
        }
    }

    pub fn update_production(&self, manager: &mut ResourceManager) {
        let transactions: Vec<Vec<ResourceTransaction>> = self.buildings
            .iter().filter(
            |(bt, v)| {
                bt.is_producing_resources() && self.buildings.get(bt).unwrap_or(&0) > &0u32
            }).map(|(bt, v)| {
            if let BuildingType::Factory(factory_type) = bt {
                factory_type.clone().into()
            } else {
                unreachable!()
            }
        }).collect();

        transactions.iter().for_each(|tr| {
            manager.apply_many(tr.clone());
        })
    }

    pub fn update_mines(
        &self,
        resource_manager: &mut ResourceManager,
        deposit: &ResourceDeposit,
        rounds: usize,
    ) {
        for _ in 0..rounds {
            for _ in 0..(*self.buildings.get(&BuildingType::Mine)
                .unwrap_or(&0) as i32) {
                resource_manager.apply(
                    ResourceTransaction::new(
                        deposit.sample(),
                        1,
                    )
                )
            }
        }
    }

    pub fn get_buildings(&self) -> Vec<(String, u32, Color)> {
        let raw: Vec<(BuildingType, Color)> = BuildingType::get_variants();

        let res: Vec<(String, u32, Color)> = raw.iter().map(|(building_type, color)|
            (
                building_type.get_name(),
                *self.buildings.get(building_type).unwrap_or(&0),
                color.clone()
            )
        ).collect();

        res
    }

    pub fn get_construction(&self) -> Vec<(String, u32)> {
        self.construction.iter().map(|p| {
            let building: String = p.building_type().clone().into();
            let progress: u32 = p.get_percentage();

            (building, progress)
        }).collect()
    }
}