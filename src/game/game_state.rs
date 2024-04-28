use std::collections::HashMap;
use std::ops::Deref;

use libc::system;
use log::info;
use ratatui::style::Color;

use crate::game::celestial_bodies::CelestialBody;
use crate::game::celestial_bodies::planet::Planet;
use crate::game::celestial_bodies::solar_system::SolarSystem;
use crate::game::research::{Research, ResearchField, ResearchProgress};

pub struct GameState {
    systems: Vec<SolarSystem>,
    research: Vec<Research>,
    pub research_progress: Vec<ResearchProgress>,
    research_fields: Vec<ResearchField>,
    capital: Planet,
    capital_system: SolarSystem,
    colonized_planets: Vec<Planet>,
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
            research: Research::load_from_file("assets/research.json5"),
            research_progress: ResearchProgress::load_from_file("assets/research_progress.json5"),
            research_fields: ResearchField::load_from_file("assets/research_fields.json5"),
            capital: capital_planet.clone(),
            capital_system: system,
            colonized_planets: vec![capital_planet]
        }
    }
}

impl GameState {
    pub fn tick(&mut self) {
        self.update_research();
    }

    pub fn new() -> Self {
        let state = Self::default();
        info!("Game state initialized");
        info!("Systems: {:?}", state.systems);
        info!("Researches: {:?}", state.research);
        info!("Research progress: {:?}", state.research_progress);
        state
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

    fn get_research_by_id(&self, id: String) -> Option<Research> {
        self.research.iter().find(|r| *r.id() == id).cloned()
    }

    fn get_research_progress_by_id(&self, id: String) -> Option<ResearchProgress> {
        self.research_progress.iter().find(|p| { p.id().clone() == id }).cloned()
    }

    fn research_requirements_satisfied(&self, research: Research) -> bool {
        let all_of: Vec<bool> = research.required_all().iter().map(
            |r| {
                let probably_progress = self.get_research_progress_by_id(r.clone());
                if let Some(progress) = probably_progress {
                    *progress.is_finished()
                } else {
                    false
                }
            }
        ).collect();

        let any_of: Vec<bool> = research.required_any().iter().map(
            |r| {
                let probably_progress = self.get_research_progress_by_id(r.clone());
                if let Some(progress) = probably_progress {
                    *progress.is_finished()
                } else {
                    false
                }
            }
        ).collect();

        if all_of.is_empty() && any_of.is_empty() {
            return true
        }

        all_of.iter().all(|r| *r) || any_of.iter().any(|r| *r)
    }
    pub fn get_research_info(&self, research: Research) -> HashMap<String, String> {
        let mut map = HashMap::from(
            [
                ("name".to_string(), research.name().clone()),
                
                ("field".to_string(), self.get_field_by_id(
                    research.field().clone()
                ).name().clone()),
                
                ("cost".to_string(), research.cost().to_string()),
                ("progress_text".to_string(), "Research locked".to_string())
            ]
        );

        if self.research_requirements_satisfied(research.clone()) {
            map.insert("progress_text".to_string(), "Research available".to_string());
        } else {
            map.insert("progress_text".to_string(), "Research locked".to_string());
        }

        if self.is_research_in_progress(research.clone()) {
            let progress_option = self.get_research_progress_by_id(research.id().clone());
            if let Some(progress) = progress_option {
                map.insert(
                    "progress".to_string(),
                    format!("{}", (*progress.progress() * 100) / *research.cost()).to_string(),
                );
                map.insert(
                    "is_finished".to_string(),
                    progress.is_finished().to_string(),
                );
                map.insert(
                    "progress_text".to_string(),
                    format!("{}% ({} / {})",
                            (*progress.progress() * 100) / *research.cost(),
                            progress.progress(),
                            research.cost()
                    ).to_string(),
                );

                if *progress.is_finished() {
                    map.insert("progress_text".to_string(), "Research completed".to_string());
                }
            }
        }

        map
    }

    fn is_research_finished(&self, research: &Research) -> bool {
        if let Some(progress) = self.get_research_progress_by_id(research.id().clone()) {
            *progress.is_finished()
        } else {
            false
        }
    }

    pub fn get_research_dependency_info(&self, research: Research) -> Vec<Vec<(String, bool)>> {
        // All of
        let mut all_of: Vec<(String, bool)> = self.research.iter()
            .filter(|r| {
                research.required_all().contains(r.id())
            })
            .cloned()
            .map(|r| {
                (r.name().clone(), self.is_research_finished(&r))
            })
            .collect();

        if all_of.is_empty() {
            all_of.push(("No technologies required".to_string(), true));
        }

        // Any of
        let mut any_of: Vec<(String, bool)> = self.research.iter()
            .filter(|r| {
                research.required_any().contains(r.id())
            })
            .cloned()
            .map(|r| {
                (r.name().clone(), self.is_research_finished(&r))
            })
            .collect();

        if any_of.is_empty() {
            any_of.push(("No technologies required".to_string(), true));
        }


        vec![all_of, any_of]
    }

    pub fn get_research_color(&self, research: Research) -> Color {
        if self.is_research_in_progress(research.clone()) {
            let progress = self.get_research_progress_by_id(research.id().clone()).unwrap();
            if *progress.is_finished() {
                Color::LightCyan
            } else {
                Color::LightYellow
            }
        } else if self.research_requirements_satisfied(research.clone()) {
            Color::White
        } else {
            Color::DarkGray
        }
    }

    pub fn update_research(&mut self) {
        if !self.research_progress.iter().any(|r| !*r.is_finished()) {
            info!("No research progress to update");
            return
        }

        let mut new_progress = Vec::<ResearchProgress>::new();
        info!("Updating researches");
        for research in self.research.as_slice() {
            if let Some(mut p) = self.get_research_progress_by_id(research.id().clone()) {
                if !p.is_finished() {
                    p.update(*research.cost());
                }
                new_progress.push(p);
            }
        }
        self.research_progress = new_progress;
    }

    pub fn start_research(&mut self, r: Research) {
        if self.research_requirements_satisfied(r.clone()) &&
            !self.is_research_in_progress(r.clone()) {
            info!("Starting research {}", r.id());
            self.research_progress.push(ResearchProgress::from(r));
            info!("{:?}", self.research_progress);
        }
    }
}
