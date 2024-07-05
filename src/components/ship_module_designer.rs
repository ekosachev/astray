use color_eyre::owo_colors::OwoColorize;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, BorderType, List, ListState};

use crate::action::Action;
use crate::components::Component;
use crate::components::utils::widget_utils::{select_next_in_list, select_prev_in_list};
use crate::tabs::Tabs;
use crate::tui::Frame;

#[derive(PartialEq)]
enum WidgetState {
    Normal,
    SelectingType,
    SelectingModule,
}

impl Default for WidgetState {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Default)]
pub struct ShipModuleDesigner {
    module_types: Vec<(String, Color)>,
    modules: Vec<(String, Color)>,
    is_initialised: bool,
    types_list_state: ListState,
    modules_list_state: ListState,
    state: WidgetState,
}

impl Component for ShipModuleDesigner {
    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        if !self.is_initialised {
            self.is_initialised = true;
            return Ok(Some(Action::ScheduleLoadShipModuleTypes));
        }

        match action {
            Action::StartSelecting => {
                if self.state == WidgetState::Normal {
                    self.state = WidgetState::SelectingType;
                    if self.types_list_state.selected().is_none() {
                        self.types_list_state.select(Some(0))
                    }
                }
            }
            Action::ContinueSelecting => {
                self.state = WidgetState::SelectingModule
                return Ok(Some(Action::))
            }
            Action::SelectNext => {
                match self.state {
                    WidgetState::Normal => {}
                    WidgetState::SelectingType => {
                        self.types_list_state.select(
                            Some(
                                select_next_in_list(
                                    self.types_list_state.selected().unwrap_or(0),
                                    self.module_types.len(),
                                )
                            )
                        )
                    }
                    WidgetState::SelectingModule => {
                        self.modules_list_state.select(
                            Some(
                                select_next_in_list(
                                    self.modules_list_state.selected().unwrap_or(0),
                                    self.modules.len(),
                                )
                            )
                        )
                    }
                }
            }
            Action::SelectPrevious => {
                match self.state {
                    WidgetState::Normal => {}
                    WidgetState::SelectingType => {
                        self.types_list_state.select(
                            Some(
                                select_prev_in_list(
                                    self.types_list_state.selected().unwrap_or(0),
                                    self.module_types.len(),
                                )
                            )
                        )
                    }
                    WidgetState::SelectingModule => {
                        self.modules_list_state.select(
                            Some(
                                select_prev_in_list(
                                    self.modules_list_state.selected().unwrap_or(0),
                                    self.modules.len(),
                                )
                            )
                        )
                    }
                }
            }
            Action::LoadShipModuleTypes(types) => { self.module_types = types }
            _ => {}
        }

        Ok(None)
    }
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> color_eyre::Result<()> {
        let v_chunks = Layout::new(
            Direction::Vertical,
            vec![
                Constraint::Length(3),
                Constraint::Fill(1),
                Constraint::Length(3),
            ],
        ).split(area);

        let a_chunks = Layout::new(
            Direction::Horizontal,
            vec![
                Constraint::Percentage(15),
                Constraint::Percentage(15),
                Constraint::Percentage(55),
                Constraint::Percentage(15),
            ],
        ).split(v_chunks[1]);

        let types_list = List::new(
            self.module_types.iter().map(
                |(i, c)| {
                    Line::styled(
                        i,
                        Style::default().fg(*c),
                    )
                }
            )
        )
            .highlight_symbol(">>")
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default()
                        .fg(if self.state == WidgetState::SelectingType {
                            Color::LightBlue
                        } else {
                            Color::White
                        }))
            );


        f.render_stateful_widget(types_list, a_chunks[0], &mut self.types_list_state);

        Ok(())
    }

    fn is_drawn_in_tab(&self, tab: &Tabs) -> bool {
        *tab == Tabs::ShipModules
    }
}