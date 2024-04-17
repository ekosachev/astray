use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, List, ListDirection, ListState};
use crate::action::Action;
use crate::components::Component;
use crate::game::celestial_bodies::{ CelestialBody, Orbitable };
use crate::game::celestial_bodies::planet::Planet;
use crate::game::celestial_bodies::solar_system::SolarSystem;
use crate::game::celestial_bodies::star::Star;
use crate::tui::Frame;

pub struct SystemTree {
    state: ListState,
    system: SolarSystem,
    is_focused: bool,
    star: Star,
    planets: Vec<Planet>,
}

impl Default for SystemTree {
    fn default() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        
        let system = SolarSystem::generate(());
        let star = system.get_star();
        let planets = system.get_satellites();
        
        Self {
            state,
            system,
            is_focused: false,
            star,
            planets,
        }
    }
}

impl Component for SystemTree {
    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        match action {
            Action::SelectBodyInSystemTree => {
                self.is_focused = true;
            }
            Action::SelectNext => {}
            Action::SelectPrevious => {}
            Action::Select => {
                self.is_focused = false;
                Ok(Some(
                    Action::UpdateObjectView()
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
                    .border_style(
                        if self.is_focused {
                            Style::default().fg(Color::LightBlue)
                        } else { 
                            Style::default()
                        }
                    )
            )
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(false)
            .direction(ListDirection::TopToBottom);

        f.render_stateful_widget(list, chunks[0], &mut self.state);

        Ok(())
    }
}