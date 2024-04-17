use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, List, ListDirection, ListState};
use crate::action::Action;
use crate::components::Component;
use crate::game::celestial_bodies::CelestialBody;
use crate::game::celestial_bodies::solar_system::SolarSystem;
use crate::tui::Frame;

pub struct SystemTree {
    state: ListState,
    system: SolarSystem,
}

impl Default for SystemTree {
    fn default() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        Self {
            state,
            system: SolarSystem::generate(()),
        }
    }
}

impl Component for SystemTree {
    // fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
    //     todo!()
    // }

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

        self.system.get_planets().iter().for_each(|p| {
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
            )
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(false)
            .direction(ListDirection::TopToBottom);

        f.render_stateful_widget(list, chunks[0], &mut self.state);

        Ok(())
    }
}