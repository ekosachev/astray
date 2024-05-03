use serde::{Deserialize, Serialize};

use crate::game::celestial_bodies::Displayable;
use crate::game::colony::building_manager::BuildingManager;
use crate::game::resource::resource::ResourceDeposit;
use crate::game::resource::resource_manager::ResourceManager;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Colony {
    planet_name: String,
    population: i32,
    building_manager: BuildingManager,
    resource_manager: ResourceManager,
    resource_deposit: ResourceDeposit,
}

impl Colony {
    pub fn new(
        planet_name: String,
        population: i32,
    ) -> Self {
        Self {
            planet_name,
            population,
            building_manager: BuildingManager::new(),
            resource_manager: ResourceManager::new(),
            resource_deposit: ResourceDeposit::generate_for_planet(),
        }
    }

    pub fn tick(&mut self) {
        self.building_manager.update_construction();
    }

    pub fn resource_tick(&mut self) {
        self.building_manager.update_mines(
            &mut self.resource_manager,
            &self.resource_deposit,
            1,
        );

        self.building_manager.update_production(
            &mut self.resource_manager,
        );
    }
}

impl Displayable for Colony {
    fn get_name(&self) -> String {
        self.planet_name.clone()
    }
}