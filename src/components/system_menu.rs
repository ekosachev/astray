use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Borders, BorderType, List, ListDirection, ListState, Paragraph};
use crate::action::Action;
use crate::components::Component;
use crate::game::celestial_bodies::{CelestialBody, Displayable, Orbitable};
use crate::game::celestial_bodies::planet::Planet;
use crate::game::celestial_bodies::solar_system::SolarSystem;
use crate::game::celestial_bodies::star::Star;
use crate::tui::Frame;

pub struct SystemMenu {
    state: ListState,
    system: SolarSystem,
    is_focused: bool,
    list_length: usize,
    text: Vec<String>,
}

impl Default for SystemMenu {
    fn default() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        
        let system = SolarSystem::generate(());

        Self {
            list_length: system.get_n_planets() + 1,
            state,
            system,
            is_focused: false,
            text: vec![],
        }
    }
}

impl Component for SystemMenu {
    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        match action {
            Action::SelectBodyInSystemTree => {
                self.is_focused = true;
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
                self.is_focused = false;

                let selected = self.state.selected().unwrap();
                if selected == 0 {
                    let star = self.system.get_star();
                    self.text = star.get_description();
                } else {
                    let planets = self.system.get_satellites();
                    self.text = planets[selected - 1].get_description();
                }
                
                return Ok(Some(
                    Action::UpdateObjectView
                ))
            }
            _ => {}
        }
        
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> color_eyre::Result<()> {
        let chunks = Layout::new(
            Direction::Horizontal,
            vec![
                Constraint::Percentage(20),
                Constraint::Fill(1),
            ]
        ).split(area);

        let mut items = Vec::<Text>::with_capacity(1 + self.system.get_n_planets());
        items.push(
            Text::styled(
                self.system.get_star().get_name(),
                Style::default().fg(self.system.get_star().get_menu_color())
            )
        );

        self.system.get_satellites().iter().for_each(|p| {
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
                    .title(self.system.get_name())
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

        let lines: Vec<Line> = self.text.iter().map(
            |x| {
                Line::from(x.as_str())
            }
        ).collect();
        
        let object_view = Paragraph::new(lines)
            .block(
                Block::default()
                    .title("Selected object")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
            );

        f.render_stateful_widget(list, chunks[0], &mut self.state);
        f.render_widget(object_view, chunks[1]);

        Ok(())
    }
}