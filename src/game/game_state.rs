use crate::game::celestial_bodies::CelestialBody;
use crate::game::celestial_bodies::solar_system::SolarSystem;
use crate::game::research::Research;

pub struct GameState {
    systems: Vec<SolarSystem>,
    research: Vec<Research>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            systems: vec![SolarSystem::generate(())],
            research: Research::load_from_file("assets/research.json5"),
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
}
