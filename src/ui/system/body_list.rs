use bevy::prelude::Resource;
use ratatui::{layout::Rect, widgets::{List, ListState}, Frame};

#[derive(Default, Resource)]
pub struct BodyList {
    pub list_state: ListState,
    pub items: Vec<String>,
    pub selected: usize,
    pub is_focused: bool,
}

pub fn render_body_list(frame: &mut Frame, area: Rect, data: &BodyList) {
    let list = List::new(data.items.clone());


    frame.render_widget(list, area);
}