use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders, BorderType, Tabs};

use crate::components::Component;
use crate::tui::Frame;

pub struct TopMenu {
    tabs: Vec<String>,
    selected: usize,
}

impl Default for TopMenu {
    fn default() -> Self {
        Self {
            tabs: vec![
                String::from("System view")
            ],
            selected: 0,
        }
    }
}

impl Component for TopMenu {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> color_eyre::Result<()> {
        let chunks = Layout::new(
            Direction::Vertical,
            vec![
                Constraint::Length(3),
                Constraint::Min(0),
            ],
        ).split(area);

        let tabs = Tabs::new(self.tabs.clone())
            .block(
                Block::default()
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL)
            )
            .select(self.selected)
            .divider("|")
            .padding("-> ", " <-");

        f.render_widget(tabs, chunks[0]);

        Ok(())
    }
}
