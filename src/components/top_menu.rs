use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders, BorderType, Tabs};

use crate::action::Action;
use crate::components::Component;
use crate::tui::Frame;

pub struct TopMenu {
    tabs: Vec<String>,
    selected: usize,
}

impl Default for TopMenu {
    fn default() -> Self {
        Self {
            tabs: vec![],
            selected: 1,
        }
    }
}

impl Component for TopMenu {
    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        match action {
            Action::LoadTabs(tabs) => {
                self.tabs = tabs.iter().map(|t| String::from(t.clone()))
                    .collect();
                self.tabs.insert(0, String::from("<Shift+Tab>"));
                self.tabs.push(String::from("<Tab>"));
            }
            Action::NavigateNextTab => {
                self.selected += 1;
                self.selected %= self.tabs.len() - 1;
                if self.selected == 0 { self.selected += 1 }
            }
            Action::NavigatePrevTab => {
                if self.selected != 1 {
                    self.selected -= 1;
                } else {
                    self.selected = self.tabs.len() - 2;
                }
            }
            _ => {}
        }

        Ok(None)
    }

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
            .padding(" == ", " == ");

        f.render_widget(tabs, chunks[0]);

        Ok(())
    }

    fn is_drawn_in_tab(&self, tab: &crate::tabs::Tabs) -> bool {
        true
    }
}
