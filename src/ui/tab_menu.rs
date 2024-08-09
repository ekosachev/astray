use std::vec;
use bevy::prelude::Resource;
use ratatui::Frame;
use ratatui::prelude::{Color, Constraint, Direction, Layout, Rect, Style};
use ratatui::widgets::{Block, Borders, Tabs};

#[derive(Resource)]
pub struct TabMenu {
    pub tab_titles: Vec<String>,
    pub selected_tab: usize,
}

pub fn render_tab_menu(frame: &mut Frame, area: Rect, data: &TabMenu) {
    let chunks = Layout::new(
        Direction::Vertical,
        vec![Constraint::Length(2), Constraint::Min(0)],
    )
        .split(area);

    let tabs = Tabs::new(data.tab_titles.clone())
        .block(Block::default().borders(Borders::BOTTOM))
        .select(data.selected_tab)
        .divider("╲  ╱")
        .padding("   ", "   ")
        .highlight_style(Style::default().fg(Color::LightGreen));

    frame.render_widget(tabs, chunks[0]);
}
