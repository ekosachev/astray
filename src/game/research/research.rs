use std::cmp::min;

use derive_getters::Getters;
use log::info;
use ratatui::prelude::Color;
use ratatui::style::Color::{LightCyan, LightGreen, LightRed, LightYellow};
use serde::{Deserialize, Serialize};

use crate::game::celestial_bodies::Displayable;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Getters)]
pub struct ResearchField {
    name: String,
    id: String,
    researches: Vec<Research>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Getters)]
pub struct Research {
    name: String,
    id: String,
    cost: u32,
    progress: u32,
    speed: u32,
    is_going: bool,
    field: String,
    required_any: Vec<String>,
    required_all: Vec<String>,
    description: String,
}

impl ResearchField {
    pub fn load_from_file(filepath: &str, research_path: &str) -> Vec<Self> {
        let mut objects: Vec<Self> = serde_json::from_str(
            &std::fs::read_to_string(filepath).unwrap()
        ).unwrap();

        let researches = Research::load_from_file(research_path);

        info!("Loaded research fields: {:?}", objects);
        info!("Loaded research objects: {:?}", researches);


        objects.iter_mut().for_each(
            |field| {
                field.researches.append(
                    &mut researches.iter().filter(|r| {
                        r.field == field.id
                    }).cloned().collect()
                )
            }
        );

        objects
    }
}

impl Research {
    pub fn load_from_file(filepath: &str) -> Vec<Self> {
        serde_json::from_str(&std::fs::read_to_string(filepath).unwrap()).unwrap()
    }

    pub fn is_finished(&self) -> bool {
        self.progress == self.cost
    }

    pub fn is_in_progress(&self) -> bool {
        self.is_going
    }

    pub fn percent_complete(&self) -> f32 {
        (self.progress as f32 * 100f32) / (self.cost as f32)
    }

    pub fn get_progress_text(&self) -> String {
        if !self.is_in_progress() && !self.is_finished() {
            return String::from("Research available")
        }

        match self.percent_complete() as i32 {
            100 => String::from("Research complete"),
            percent => format!(
                "Research in progress: {}% ({}/{})",
                percent,
                self.progress,
                self.cost,
            )
        }
    }

    pub fn start(&mut self) {
        self.is_going = true;
    }
}

impl ResearchField {
    pub fn finished_research(&self) -> Vec<Research> {
        self.researches.iter().filter(|r| { r.is_finished() }).cloned().collect()
    }

    pub fn get_id(&self) -> String { self.id.clone() }
    pub fn get_researches(&self) -> Vec<Research> { self.researches.clone() }

    pub fn has_research_with_id(&self, id: String) -> bool {
        self.researches.iter().any(
            |r| { r.id == id }
        )
    }

    pub fn get_research_by_id(&self, id: String) -> &Research {
        self.researches.iter().find(
            |r| {
                r.id == id
            }
        ).unwrap()
    }

    pub fn get_mut_research_by_id(&mut self, id: String) -> &mut Research {
        self.researches.iter_mut().find(
            |r| r.id == id
        ).unwrap()
    }

    pub fn tick(&mut self) {
        for r in self.researches.as_mut_slice() {
            if r.is_in_progress() {
                r.progress = min(r.cost, r.progress + r.speed)
            }
            if r.progress == r.cost {
                r.is_going = false
            }
        }
    }
}

impl Displayable for ResearchField {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_menu_color(&self) -> Color {
        let total_research = self.researches.len() as f32;
        let finished_research = self.finished_research().len() as f32;

        if total_research == 0.0 {
            return Color::Gray;
        } else {
            match (finished_research * 100f32 / total_research) as i32 {
                0..=10 => LightRed,
                11..=50 => LightYellow,
                51..=90 => LightGreen,
                _ => LightCyan,
            }
        }
    }
}

impl Displayable for Research {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_properties(&self) -> Vec<Vec<String>> {
        vec![
            vec![
                self.name.clone()
            ],
            vec![
                format!(
                    "Cost: {} points", self.cost
                )
            ],
            vec![
                format!(
                    "Description: {}", self.description.clone()
                )
            ],
        ]
    }

    fn get_menu_color(&self) -> Color {
        match self.percent_complete() as i32 {
            0..=25 => LightRed,
            25..=75 => LightYellow,
            76..=99 => LightGreen,
            _ => LightCyan,
        }
    }
}