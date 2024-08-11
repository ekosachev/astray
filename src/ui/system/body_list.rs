use bevy::prelude::Resource;
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{List, ListState},
};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders};

#[derive(Default, Resource)]
pub struct BodyList {
    pub list_state: ListState,
    pub items: Vec<String>,
    pub is_focused: bool,
}

pub fn render_body_list(frame: &mut Frame, area: Rect, data: &mut BodyList) {
    let list = List::new(data.items.clone())
        .highlight_symbol(">")
        .highlight_style(if data.is_focused {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default().fg(Color::LightBlue)
        })
        .block(Block::default().borders(Borders::RIGHT));

    frame.render_stateful_widget(list, area, &mut data.list_state);
}
