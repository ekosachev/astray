use color_eyre::owo_colors::OwoColorize;
use log::info;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::symbols::Marker;
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, BorderType, List, ListDirection, ListState, Row, Table};
use ratatui::widgets::canvas::Canvas;

use crate::action::Action;
use crate::components::Component;
use crate::game::celestial_bodies::{Displayable, Orbitable};
use crate::game::celestial_bodies::solar_system::SolarSystem;
use crate::tabs::Tabs;
use crate::tui::Frame;

pub struct SystemMenu {
    state: ListState,
    system: Option<SolarSystem>,
    is_focused: bool,
    map_focused: bool,
    list_length: usize,
    properties: Vec<Vec<String>>,
    map_shift_x: f64,
    map_shift_y: f64,
    map_zoom: f64,
}

impl Default for SystemMenu {
    fn default() -> Self {
        let mut state = ListState::default();
        state.select(None);

        Self {
            list_length: 0,
            state,
            system: None,
            is_focused: false,
            map_focused: false,
            properties: vec![],
            map_shift_x: 0.0,
            map_shift_y: 0.0,
            map_zoom: 1.0,
        }
    }
}

impl SystemMenu {
    pub fn set_system(&mut self, system: SolarSystem) {
        self.list_length = 1 + system.get_n_planets();
        self.state.select(Some(0));
        self.system = Some(system);
    }
}

impl Component for SystemMenu {
    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        match action {
            Action::IngameTick => {
                return Ok(Some(Action::ScheduleLoadSystemView))
            }
            Action::StartSelecting => {
                self.is_focused = true;
            }
            Action::SecondaryAction => {
                self.map_focused = true;
                return Ok(Some(Action::EnterSystemMapNavigation))
            }
            Action::LoadSystemView(system) => {
                self.set_system(system);

            }
            Action::SelectNext => {
                let selected = self.state.selected().unwrap();
                if selected == (self.list_length - 1) {
                    self.state.select(Some(0))
                } else {
                    self.state.select(Some(selected + 1))
                }
            }
            Action::SelectPrevious => {
                let selected = self.state.selected().unwrap();
                if selected == 0 {
                    self.state.select(Some(self.list_length - 1))
                } else {
                    self.state.select(Some(selected - 1))
                }
            }
            Action::Select => {
                if self.is_focused {
                    self.is_focused = false;

                    let selected = self.state.selected().unwrap();
                    if selected == 0 {
                        let star = self.system.clone().unwrap().get_star();
                        self.properties = star.get_properties();
                    } else {
                        let planets = self.system.clone().unwrap().get_satellites();
                        self.properties = planets[selected - 1].get_properties();
                    }

                    return Ok(Some(
                        Action::UpdateObjectView
                    ))
                } else if self.map_focused {
                    self.map_focused = false;
                }
            }
            Action::Up => {
                self.map_shift_y -= 2.0 * self.map_zoom
            }
            Action::Down => {
                self.map_shift_y += 2.0 * self.map_zoom
            }
            Action::Left => {
                self.map_shift_x += 2.0 * self.map_zoom
            }
            Action::Right => {
                self.map_shift_x -= 2.0 * self.map_zoom
            }
            Action::ZoomIn => {
                self.map_zoom -= 0.1
            }
            Action::ZoomOut => {
                self.map_zoom += 0.1
            }
            _ => {}
        }
        
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> color_eyre::Result<()> {
        if self.system.is_none() {
            return Ok(())
        }

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
                Constraint::Fill(1),
            ]
        ).split(v_chunks[1]);

        let s_chunks = Layout::new(
            Direction::Vertical,
            vec![
                Constraint::Fill(1),
                Constraint::Max(10),
            ],
        ).split(chunks[1]);

        let mut items = Vec::<Text>::with_capacity(1 + self.system.clone().unwrap().get_n_planets());
        items.push(
            Text::styled(
                self.system.clone().unwrap().get_star().get_name(),
                Style::default().fg(self.system.clone().unwrap().get_star().get_menu_color())
            )
        );

        self.system.clone().unwrap().get_satellites().iter().for_each(|p| {
           items.push(
               Text::styled(
                   p.get_name(),
                   Style::default().fg(p.get_menu_color())
               )
           )
        });

        let list = List::new(items)
            .block(
                Block::default()
                    .title(self.system.clone().unwrap().get_name())
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

        let mut rows: Vec<Row> = Vec::with_capacity(self.properties.len());

        for property in self.properties.iter() {
            rows.push(Row::new(property.clone()));
        }
        
        let widths = vec![
            Constraint::Fill(2),
            Constraint::Fill(3),
            Constraint::Fill(3),
        ];
        
        let object_view = Table::new(rows, widths)
            .header(Row::new(vec!["Property", "Value", "Value in relative units"])
                .style(Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD)))
            .block(
                Block::default()
                    .title("Selected object")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
            );

        let height = [
            (-15f64 + self.map_shift_x) * self.map_zoom,
            (15f64 + self.map_shift_x) * self.map_zoom
        ];
        let aspect_ratio = (s_chunks[0].width as f64) / (s_chunks[0].height as f64) / 2.0;
        let width = [
            ((-15f64 + self.map_shift_y) / aspect_ratio) * self.map_zoom ,
            ((15f64 + self.map_shift_y) / aspect_ratio) * self.map_zoom,
        ];


        let system_image = Canvas::default()
            .block(
                Block::default()
                    .title("System")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(
                        if self.map_focused {
                            Style::default().fg(Color::LightBlue)
                        } else {
                            Style::default()
                        }
                    )
            )
            .x_bounds(height)
            .y_bounds(width)
            .paint(|ctx| {
                if let Some(system) = self.system.clone() {
                    system.draw_image(
                        ctx,
                    )
                }
            });

        f.render_stateful_widget(list, chunks[0], &mut self.state);
        f.render_widget(object_view, s_chunks[1]);
        f.render_widget(system_image, s_chunks[0]);

        Ok(())
    }

    fn is_drawn_in_tab(&self, tab: &Tabs) -> bool {
        *tab == Tabs::SystemView
    }
}