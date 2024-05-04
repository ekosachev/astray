use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets;
use ratatui::widgets::{Block, Borders, BorderType, ListDirection, ListState, Paragraph};

use crate::action::Action;
use crate::components::Component;
use crate::components::utils::widget_utils;
use crate::game::celestial_bodies::Displayable;
use crate::game::colony::colony::Colony;
use crate::tabs::Tabs;
use crate::tui::Frame;

pub struct ColoniesMenu {
    colonies: Vec<Colony>,
    list_state: ListState,
    is_focused: bool,
    selected_colony: Option<Colony>,
}

impl Default for ColoniesMenu {
    fn default() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        Self {
            colonies: Vec::new(),
            list_state: state,
            is_focused: false,
            selected_colony: None,
        }
    }
}

impl Component for ColoniesMenu {
    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        match action {
            Action::LoadColonies(colonies) => {
                self.colonies = colonies;
                self.list_state.select(Some(0));
            },
            Action::StartSelecting => {
                self.is_focused = true
            },
            Action::SelectPrevious => {
                self.list_state.select(Some(
                    widget_utils::select_prev_in_list(
                        self.list_state.selected().unwrap(),
                        self.colonies.len(),
                    )
                ))
            },
            Action::SelectNext => {
                self.list_state.select(Some(
                    widget_utils::select_next_in_list(
                        self.list_state.selected().unwrap(),
                        self.colonies.len(),
                    )
                ))
            },
            Action::Select => {
                self.selected_colony = Some(
                    self.colonies[self.list_state.selected().unwrap()].clone()
                );
                self.is_focused = false;
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
            Direction::Vertical,
            vec![
                Constraint::Min(0)
            ],
        ).split(h_chunks[1]);


        let colonies_list = widgets::List::new(self.colonies.iter().map(
            |c| { c.get_name() }
        ))
            .block(
                Block::default()
                    .title("Fields")
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

        let colony_info = if let Some(colony) = &self.selected_colony {
            let mut lines = vec![
                Line::from(format!("Colony: {}", colony.get_name())),
                Line::from(format!("Population: {}", colony.get_population())),
            ];

            lines.append(&mut colony.get_buildings().iter().map(
                |(name, amount)| {
                    Line::from(vec![
                        Span::from(format!("{}: ", name)),
                        Span::styled(
                            amount.to_string(),
                            Style::default()
                                .add_modifier(Modifier::ITALIC)
                                .fg(if *amount == 0 {
                                    Color::DarkGray
                                } else {
                                    Color::LightBlue
                                }),
                        ),
                    ])
                }
            ).collect());
            Paragraph::new(lines)
        } else {
            Paragraph::new("Select a colony")
        }
            .block(
                Block::default()
                    .title("Colony")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
            );

        f.render_stateful_widget(colonies_list, h_chunks[0], &mut self.list_state);
        f.render_widget(colony_info, p_chunks[0]);

        Ok(())
    }

    fn is_drawn_in_tab(&self, tab: &Tabs) -> bool {
        *tab == Tabs::Colonies
    }
}