use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets;
use ratatui::widgets::{Block, Borders, BorderType, ListDirection, ListState, Paragraph};

use crate::action::Action;
use crate::components::Component;
use crate::components::utils::widget_utils;
use crate::tabs::Tabs;
use crate::tui::Frame;

/// `ColoniesMenu` is a struct that represents a tab, that can be used by the player to manage
/// colonies
///
/// **Fields**
/// - is_initialised (`bool`) - is used for initial data loading, `false` if data was never
/// loaded, `true` otherwise
/// - colonies (`Vec<String>`) - holds a list of colonies (by names)
/// - list_state (`ListState`) - holds the current state of the colonies list widget
/// - selected_colony (`Option<String>`) - holds the name of the currently selected colony or
/// `None` if no colony is selected
/// - is_focused (`bool`) - `true` if the colonies list is active, `false` otherwise, used for 
/// the border color of said list
/// - is_building_focused (`bool`) - `true` is the building list is selected, `false` otherwise,
/// used for the border color
/// - buildings_list_state (`ListState`) - holds the current state of the buildings list widget
/// - buildings_list (`Vec<(String, u32, Color)>`) - holds the entries of the buildings list:
///     - `String` - name of the building
///     - `u32` - amount of buildings
///     - `Color` - color of the list entry
/// - info (`Vec<(String, Color)>`) - holds the information about the currently selected colony, 
/// separated into individual lines:
///     - `String` - text
///     - `Color` - color it should be displayed with
/// - construction_info (`Vec<(String, u32)>`) - holds information about active and scheduled 
/// construction projects:
///     - `String` - name of the building
///     - `u32` - progress in %
pub struct ColoniesMenu {
    is_initialised: bool,
    colonies: Vec<String>,
    list_state: ListState,
    selected_colony: Option<String>,
    is_focused: bool,
    is_building_focused: bool,
    buildings_list_state: ListState,
    buildings_list: Vec<(String, u32, Color)>,
    info: Vec<(String, Color)>,
    construction_info: Vec<(String, u32)>
}

impl Default for ColoniesMenu {
    fn default() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        Self {
            is_initialised: false,
            colonies: Vec::new(),
            list_state: state,
            is_focused: false,
            is_building_focused: false,
            selected_colony: None,
            buildings_list_state: ListState::default(),
            buildings_list: vec![(String::from("Select a colony"), 0, Color::Red)],
            info: vec![(String::from("Select a colony"), Color::Red)],
            construction_info: vec![(String::from("Select a colony"), 0)]
        }
    }
}

impl Component for ColoniesMenu {
    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        if !self.is_initialised {
            self.is_initialised = true;
            return Ok(Some(Action::InitColonies))
        }

        match action {
            Action::LoadColonies(colonies) => {
                self.colonies = colonies;
                self.list_state.select(Some(0));
            },
            Action::LoadColonyInfo(new_info) => {
                self.info = new_info;
            }
            Action::LoadColonyBuildings(data) => {
                self.buildings_list = data;
            }
            Action::StartSelecting => {
                self.is_focused = true
            },
            Action::SelectPrevious => {
                if self.is_focused {
                    self.list_state.select(Some(
                        widget_utils::select_prev_in_list(
                            self.list_state.selected().unwrap(),
                            self.colonies.len(),
                        )
                    ))
                } else if self.is_building_focused {
                    self.buildings_list_state.select(Some(
                        widget_utils::select_prev_in_list(
                            self.buildings_list_state.selected().unwrap(),
                            self.buildings_list.len(),
                        )
                    ))
                }
            },
            Action::SelectNext => {
                if self.is_focused {
                    self.list_state.select(Some(
                        widget_utils::select_next_in_list(
                            self.list_state.selected().unwrap(),
                            self.colonies.len(),
                        )
                    ))
                } else if self.is_building_focused {
                    self.buildings_list_state.select(Some(
                        widget_utils::select_next_in_list(
                            self.buildings_list_state.selected().unwrap(),
                            self.buildings_list.len(),
                        )
                    ))
                }
            },
            Action::Select => {
                if self.is_focused {
                    let selected_name = self.colonies[
                        self.list_state.selected().unwrap()
                        ].clone();
                    self.selected_colony = Some(selected_name.clone());
                    self.is_focused = false;
                    return Ok(
                        Some(Action::ScheduleLoadColonyInfo(selected_name))
                    )
                } else if self.is_building_focused {
                    self.is_building_focused = false;

                    let selected_building = self.buildings_list[
                        self.buildings_list_state.selected().unwrap()
                        ].clone().0;

                    self.buildings_list_state.select(None);
                    return Ok(
                        Some(
                            Action::StartConstruction(
                                (
                                    self.selected_colony.clone().unwrap(),
                                    selected_building
                                )
                            )
                        )
                    )
                }
            },
            Action::MainAction => {
                return Ok(Some(
                    Action::StartSelectingBuilding
                ))
            },
            Action::StartSelectingBuilding => {
                self.buildings_list_state.select(Some(0));
                self.is_building_focused = true;
            },
            Action::IngameTick => {
                if let Some(colony_name) = self.selected_colony.clone() {
                    return Ok(
                        Some(Action::ScheduleLoadConstructionInfo(colony_name))
                    )
                }
            }
            Action::LoadConstructionInfo(data) => {
                self.construction_info = data;
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

        let h_chunks = Layout::new(
            Direction::Horizontal,
            vec![
                Constraint::Percentage(20),
                Constraint::Percentage(80),
            ],
        ).split(v_chunks[1]);

        let p_chunks = Layout::new(
            Direction::Horizontal,
            vec![
                Constraint::Percentage(60),
                Constraint::Percentage(40),
            ],
        ).split(h_chunks[1]);


        let colonies_list = widgets::List::new(self.colonies.clone())
            .block(
                Block::default()
                    .title("Colonies")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(
                        if self.is_focused {
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

        let lines: Vec<Line> = self.info.iter().map(|(text, color)| {
            Line::from(
                Span::styled(
                    text,
                    Style::default().fg(*color),
                )
            )
        }).collect();

        let colony_info = Paragraph::new(lines)
            .block(
                Block::default()
                    .title("Information")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
            );

        let b_chunks = Layout::new(
            Direction::Vertical,
            vec![
                Constraint::Fill(1),
                Constraint::Length(10),
            ],
        ).split(p_chunks[1]);


        let entries: Vec<Line> = self.buildings_list.iter().map(
            |(building, amount, color)| {
                Line::styled(
                    format!("{}: {}", building, amount),
                    Style::default().fg(*color),
                )
            }
        ).collect();

        let buildings_list = widgets::List::new(entries)
            .block(
                Block::default()
                    .title("Buildings")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(
                        if self.is_building_focused {
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


        f.render_stateful_widget(buildings_list, b_chunks[0], &mut self.buildings_list_state);


        let entries: Vec<Line> = self.construction_info.iter().map(
            |(name, progress)| {
                Line::styled(
                    format!(
                        "{}: {}%",
                        name,
                        progress,
                    ),
                    Style::default().fg(
                        match progress {
                            0..=25 => Color::LightRed,
                            26..=50 => Color::LightYellow,
                            51..=75 => Color::LightGreen,
                            76..=100 => Color::LightCyan,
                            _ => Color::White
                        }
                    ),
                )
            }
        ).collect();

        let construction_list = widgets::List::new(entries)
            .block(
                Block::default()
                    .title("Construction")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
            )
            .direction(ListDirection::TopToBottom);

        f.render_widget(construction_list, b_chunks[1]);

        f.render_stateful_widget(colonies_list, h_chunks[0], &mut self.list_state);
        f.render_widget(colony_info, p_chunks[0]);

        Ok(())
    }

    fn is_drawn_in_tab(&self, tab: &Tabs) -> bool {
        *tab == Tabs::Colonies
    }
}