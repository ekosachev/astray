use crate::game::colony::building_manager::BuildingManager;

pub struct Colony {
    planet_name: String,
    population: i32,
    building_manager: BuildingManager
}

impl Colony {
    pub fn new(
        planet_name: String,
        population: i32,
        building_manager: BuildingManager) -> Self {
        Self {
            planet_name,
            population,
            building_manager,
        }
    }
}