use ratatui::style::Color;

use crate::game::celestial_bodies::{CelestialBody, Displayable};
use crate::game::celestial_bodies::planet::Planet;
use crate::game::celestial_bodies::solar_system::SolarSystem;
use crate::game::colony::building::BuildingType;
use crate::game::colony::colony::Colony;
use crate::game::research::research_manager::ResearchManager;

pub struct GameState {
    systems: Vec<SolarSystem>,
    capital: Planet,
    capital_system: SolarSystem,
    colonies: Vec<Colony>,
    resource_tick_ratio: u32,
    resource_tick_counter: u32,
    research_manager: ResearchManager,
}

impl Default for GameState {
    fn default() -> Self {
        let mut system: SolarSystem;
        let capital_planet: Planet;
        loop {
            system = SolarSystem::generate(());
            if let Some(planet) = system.has_planets_in_habitable_zone() {
                capital_planet = planet;
                break
            }
        }

        Self {
            systems: vec![system.clone()],
            capital: capital_planet.clone(),
            capital_system: system,
            colonies: vec![
                Colony::new(
                    capital_planet.get_name(),
                    5_000,
                )
            ],
            resource_tick_counter: 0,
            resource_tick_ratio: 2,
            research_manager: ResearchManager::new(),
        }
    }
}

impl GameState {
    pub fn tick(&mut self) {
        self.update_research();
        self.update_colonies();
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_starting_system(&self) -> SolarSystem {
        self.systems[0].clone()
    }

    pub fn get_research_fields(&self) -> Vec<(String, String, Color)> {
        self.research_manager.get_research_fields()
    }

    pub fn get_research_info(&self, id: String) -> Vec<Vec<String>> {
        self.research_manager.get_research_info(id)
    }

    pub fn get_research_dependency_info(&self, id: String) -> Vec<Vec<(String, bool)>> {
        self.research_manager.get_dependency_info(id)
    }

    pub fn get_researches_by_field(&self, id: String) -> Vec<(String, String, Color)> {
        self.research_manager.get_researches_with_colors_by_field(id)
    }

    pub fn get_research_progress_text(&self, id: String) -> String {
        self.research_manager.get_research_text(id)
    }

    pub fn get_research_progress(&self, id: String) -> u32 {
        self.research_manager.get_research_progress(id)
    }

    pub fn start_research(&mut self, id: String) {
        self.research_manager.start_research(id)
    }

    fn update_research(&mut self) {
        self.research_manager.tick();
    }

    fn update_colonies(&mut self) {
        self.colonies.iter_mut().for_each(|c| c.tick());
        self.resource_tick_counter += 1;
        if self.resource_tick_ratio == self.resource_tick_counter {
            self.resource_tick_counter = 0;
            self.colonies.iter_mut().for_each(|c| c.resource_tick());
        }
    }

    pub fn get_colonies(&self) -> Vec<Colony> {
        self.colonies.clone()
    }

    pub fn start_construction(&mut self, colony: Colony, building: BuildingType) {
        self.colonies.iter_mut().find(|c| c == &&colony).unwrap()
            .start_construction(building)
    }

    pub fn get_colony_by_name(&self, name: String) -> Option<Colony> {
        self.colonies.iter().find(|c| c.get_name() == name).cloned()
    }
}
