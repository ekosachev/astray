use std::collections::HashMap;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets;
use ratatui::widgets::{Block, Borders, BorderType, ListDirection, ListState};
use crate::action::Action;

use crate::components::Component;
use crate::components::utils::widget_utils;
use crate::game::research::{Research, ResearchField};
use crate::tabs::Tabs;
use crate::tui::Frame;

pub struct ResearchMenu {
    field_list_state: ListState,
    research_list_state: ListState,
    field_list: Vec<ResearchField>,
    research_list: Vec<Research>,
    field_list_focused: bool,
    research_list_focused: bool,
    info: HashMap<String, String>
}

impl Default for ResearchMenu {
    fn default() -> Self {
        let mut field_list_state = ListState::default();
        field_list_state.select(Some(0));
        let mut research_list_state = ListState::default();
        research_list_state.select(Some(0));

        Self {
            field_list_state,
            research_list_state,
            field_list: Vec::new(),
            research_list: Vec::new(),
            field_list_focused: false,
            research_list_focused: false,
            info: HashMap::new(),
        }
    }
}

impl Component for ResearchMenu {
    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        match action {
            Action::LoadResearchFields(fields) => {
                self.field_list = fields;
            }
            
            Action::StartSelecting => {
                self.field_list_focused = true;
            }
            
            Action::SelectPrevious => {
                if self.field_list_focused {
                    self.field_list_state.select(Some(
                        widget_utils::select_prev_in_list(
                            self.field_list_state.selected().unwrap(),
                            self.field_list.len()
                        )
                    ))
                } else if self.research_list_focused {
                    self.research_list_state.select(Some(
                        widget_utils::select_prev_in_list(
                            self.research_list_state.selected().unwrap(),
                            self.research_list.len()
                        )
                    ))
                }
            }
            
            Action::SelectNext => {
                if self.field_list_focused {
                    self.field_list_state.select(Some(
                        widget_utils::select_next_in_list(
                            self.field_list_state.selected().unwrap(),
                            self.field_list.len()
                        )
                    ))
                } else if self.research_list_focused {
                    self.research_list_state.select(Some(
                        widget_utils::select_next_in_list(
                            self.research_list_state.selected().unwrap(),
                            self.research_list.len()
                        )
                    ))
                }
            }
            
            Action::ContinueSelecting => {
                self.research_list_focused = true;
                self.field_list_focused = false;
                return Ok(Some(
                    Action::ScheduleLoadResearchesForField(
                        self.field_list[self.field_list_state.selected().unwrap()].clone()
                    )
                ))
            }
            
            Action::Select => {
                self.research_list_focused = false;
                if self.research_list.len() > 0 {
                    return Ok(Some(
                        Action::ScheduleLoadResearchInfo(
                            self.research_list[self.research_list_state.selected().unwrap()].clone()
                        )
                    ))
                }
            }
            
            Action::LoadResearchesForField(researches) => {
                self.research_list = researches;
            }
            
            Action::LoadResearchInfo(info) => {
                self.info = info;
            }
            
            _ => {}
        }
        
        Ok(None)
    }
    
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> color_eyre::Result<()> {
        let v_chunks = Layout::new(
            Direction::Vertical,
            vec![
                Constraint::Length(3),
                Constraint::Min(0),
            ],
        ).split(area);

        let chunks = Layout::new(
            Direction::Horizontal,
            vec![
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(60),
            ],
        ).split(v_chunks[1]);

        let fields_list = widgets::List::new(self.field_list.iter().map(
            |r| { String::from(r.clone()) }
        ))
            .block(
                Block::default()
                    .title("Fields")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(
                        if self.field_list_focused {
                            Style::default().fg(Color::LightBlue)
                        } else {
                            Style::default()
                        }
                    )
            )
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">> ")
            .repeat_highlight_symbol(false)
            .direction(ListDirection::TopToBottom);

        let research_list = widgets::List::new(self.research_list.iter().map(
            |r| { r.name().clone() }
        ))
            .block(
                Block::default()
                    .title("Researches")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(
                        if self.research_list_focused {
                            Style::default().fg(Color::LightBlue)
                        } else {
                            Style::default()
                        }
                    )
            )
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">> ")
            .repeat_highlight_symbol(false)
            .direction(ListDirection::TopToBottom);

        f.render_stateful_widget(fields_list, chunks[0], &mut self.field_list_state);
        f.render_stateful_widget(research_list, chunks[1], &mut self.research_list_state);

        Ok(())
    }

    fn is_drawn_in_tab(&self, tab: &Tabs) -> bool {
        *tab == Tabs::Research
    }
}