use ratatui::style::Color;
use ratatui::style::Color::{DarkGray, LightCyan};

use crate::game::celestial_bodies::Displayable;
use crate::game::research::research::{Research, ResearchField};

pub struct ResearchManager {
    research_fields: Vec<ResearchField>,
}

impl Default for ResearchManager {
    fn default() -> Self {
        Self {
            research_fields: ResearchField::load_from_file(
                "assets/research_fields.json5",
                "assets/research.json5",
            ),
        }
    }
}

impl ResearchManager {
    pub fn new() -> Self { Self::default() }

    pub fn get_research_fields(&self) -> Vec<(String, String, Color)> {
        self.research_fields.iter().map(
            |r| {
                (r.id().clone(), r.get_name(), r.get_menu_color())
            }
        ).collect()
    }

    fn get_field_by_id(&self, id: String) -> &ResearchField {
        self.research_fields.iter().find(
            |rf| { rf.get_id() == id }
        ).unwrap()
    }

    fn get_mut_field_by_id(&mut self, id: String) -> &mut ResearchField {
        self.research_fields.iter_mut().find(
            |rf| rf.get_id() == id
        ).unwrap()
    }

    fn get_researches_by_field(&self, id: String) -> Vec<Research> {
        let field = self.get_field_by_id(id);
        field.get_researches()
    }

    fn get_research_by_id(&self, id: String) -> &Research {
        let field = self.research_fields
            .iter()
            .find(|f| f.has_research_with_id(id.clone()))
            .unwrap();

        field.get_research_by_id(id)
    }


    fn are_research_requirements_satisfied(&self, id: String) -> bool {
        let all_of: Vec<bool> = self.get_research_by_id(id.clone())
            .required_all()
            .iter()
            .map(
                |id| {
                    self.get_research_by_id(id.clone()).is_finished()
                }
            )
            .collect();

        let any_of: Vec<bool> = self.get_research_by_id(id.clone())
            .required_any()
            .iter()
            .map(
                |id| {
                    self.get_research_by_id(id.clone()).is_finished()
                }
            )
            .collect();

        (all_of.is_empty() || all_of.iter().all(|i| *i)) &&
            (any_of.is_empty() || any_of.iter().any(|i| *i))
    }

    pub fn get_research_text(&self, id: String) -> String {
        let research = self.get_research_by_id(id);
        if self.are_research_requirements_satisfied(research.id().clone()) {
            research.get_progress_text()
        } else {
            String::from("Research locked")
        }
    }

    pub fn get_all_researches(&self) -> Vec<Research> {
        let mut res = Vec::<Research>::new();

        self.research_fields.iter().for_each(
            |rf| {
                res.append(&mut rf.get_researches())
            }
        );

        res
    }

    pub fn get_dependency_info(&self, id: String) -> Vec<Vec<(String, bool)>> {
        let research = self.get_research_by_id(id);
        let research_list = self.get_all_researches();
        // All of
        let mut all_of: Vec<(String, bool)> = research_list.iter()
            .filter(|r| {
                research.required_all().contains(r.id())
            })
            .cloned()
            .map(|r| {
                (r.name().clone(), r.is_finished())
            })
            .collect();

        if all_of.is_empty() {
            all_of.push(("No technologies required".to_string(), true));
        }

        // Any of
        let mut any_of: Vec<(String, bool)> = research_list.iter()
            .filter(|r| {
                research.required_any().contains(r.id())
            })
            .cloned()
            .map(|r| {
                (r.name().clone(), r.is_finished())
            })
            .collect();

        if any_of.is_empty() {
            any_of.push(("No technologies required".to_string(), true));
        }


        vec![all_of, any_of]
    }

    pub fn get_research_color(&self, id: String) -> Color {
        let research = self.get_research_by_id(id.clone());
        if research.is_finished() {
            LightCyan
        } else if !self.are_research_requirements_satisfied(id) {
            DarkGray
        } else {
            research.get_menu_color()
        }
    }

    fn get_mut_research_by_id(&mut self, id: String) -> &mut Research {
        let research = self.get_research_by_id(id.clone());
        let field = self.get_mut_field_by_id(research.field().clone());
        field.get_mut_research_by_id(id)
    }

    pub fn start_research(&mut self, id: String) {
        self.get_mut_research_by_id(id.clone()).start();
    }

    pub fn tick(&mut self) {
        for rf in self.research_fields.as_mut_slice() {
            rf.tick();
        }
    }

    pub fn get_research_info(&self, id: String) -> Vec<Vec<String>> {
        let research = self.get_research_by_id(id);
        research.get_properties()
    }

    pub fn get_researches_with_colors_by_field(&self, id: String) -> Vec<(String, String, Color)> {
        let field = self.get_field_by_id(id);
        let res: Vec<(String, String, Color)> = field.researches().clone().iter()
            .map(|x| {
                (x.id().clone(), x.get_name(), self.get_research_color(x.id().clone()))
            }).collect();

        res
    }

    pub fn get_research_progress(&self, id: String) -> u32 {
        self.get_research_by_id(id).percent_complete() as u32
    }
}