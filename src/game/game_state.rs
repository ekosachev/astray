use std::collections::HashMap;
use crate::game::celestial_bodies::CelestialBody;
use crate::game::celestial_bodies::solar_system::SolarSystem;
use crate::game::research::{Research, ResearchField, ResearchProgress};

pub struct GameState {
    systems: Vec<SolarSystem>,
    research: Vec<Research>,
    research_progress: Vec<ResearchProgress>,
    research_fields: Vec<ResearchField>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            systems: vec![SolarSystem::generate(())],
            research: Research::load_from_file("assets/research.json5"),
            research_progress: ResearchProgress::load_from_file("assets/research_progress.json5"),
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
    pub fn get_field_by_id(&self, id: String) -> ResearchField {
        self.research_fields.iter().find(|f| {
            f.id().clone() == id
        }).unwrap().clone()
    }
    pub fn get_researches(&self) -> Vec<Research> { self.research.clone() }
    
    pub fn get_researches_by_field(&self, field: ResearchField) -> Vec<Research> {
        self.research.iter().filter(|r| r.field() == field.id()).cloned().collect()
    }
    
    fn is_research_in_progress(&self, research: Research) -> bool {
        self.research_progress.iter().any(|p| p.id() == research.id())
    }
    
    fn get_research_progress_by_id(&self, id: String) -> ResearchProgress {
        self.research_progress.iter().find(|p| { p.id().clone() == id }).unwrap().clone()
    }
    
    pub fn get_research_info(&self, research: Research) -> HashMap<String, String> {
        let mut map = HashMap::from(
            [
                ("name".to_string(), research.name().clone()),
                
                ("field".to_string(), self.get_field_by_id(
                    research.field().clone()
                ).name().clone()),
                
                ("cost".to_string(), research.cost().to_string()),
            ]
        );
        
        if self.is_research_in_progress(research.clone()) {
            let progress = self.get_research_progress_by_id(research.id().clone());
            map.insert(
                "progress".to_string(),
                progress.progress().to_string()
            );
            map.insert(
                "is_finished".to_string(),
                progress.is_finished().to_string()
            );
        }
        
        map
    }
}
