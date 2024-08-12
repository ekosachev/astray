use bevy::prelude::EventWriter;
use ratatui::Frame;
use ratatui::prelude::Rect;
use ratatui::widgets::{Block, Borders};
use ratatui::widgets::canvas::Canvas;
use crate::components::general::{Orbit, Position, Radius};
use crate::InGameEvent;

pub struct SystemDisplay {
    pub translation: Position,
    pub zoom: f32,
}

pub fn render_system_display(frame: &mut Frame, area: Rect, data: &SystemDisplay, star: (Position, Radius), planets: Vec<(Orbit, Position, Radius)>) {
    let display = Canvas::default()
        .block(Block::default().borders(Borders::BOTTOM))
        .paint(|ctx| {

        })

    frame.render_widget(display, area);
}