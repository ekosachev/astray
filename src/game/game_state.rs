use crate::game::celestial_bodies::CelestialBody;
use crate::game::celestial_bodies::solar_system::SolarSystem;
use crate::game::research::{Research, ResearchField};

pub struct GameState {
    systems: Vec<SolarSystem>,
    research: Vec<Research>,
    research_fields: Vec<ResearchField>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            systems: vec![SolarSystem::generate(())],
            research: Research::load_from_file("assets/research.json5"),
            research_fields: ResearchField::load_from_file("assets/research_fields.json5"),
        }
    }
}

impl GameState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_starting_system(&self) -> SolarSystem {
        self.systems[0].clone()
    }
    
    pub fn get_research_fields(&self) -> Vec<ResearchField> { self.research_fields.clone() }
    pub fn get_researches(&self) -> Vec<Research> { self.research.clone() }
}
