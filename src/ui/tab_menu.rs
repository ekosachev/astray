use std::vec;

use ratatui::Frame;
use ratatui::prelude::{Color, Constraint, Direction, Layout, Rect, Style};
use ratatui::widgets::{Block, Borders, Tabs};

pub fn render_tab_menu(frame: &mut Frame, area: Rect) {
    let chunks = Layout::new(
        Direction::Vertical,
        vec![Constraint::Length(2), Constraint::Min(0)],
    )
    .split(area);

    let tabs = Tabs::new(vec!["System", "Science", "Colonies", "Ship modules"])
        .block(Block::default().borders(Borders::BOTTOM))
        .select(0)
        .divider("╲  ╱")
        .padding("   ", "   ")
        .highlight_style(Style::default().fg(Color::LightGreen));

    frame.render_widget(tabs, chunks[0]);
}
