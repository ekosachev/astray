use bevy::prelude::{AppExit, EventReader, EventWriter};
use bevy_ratatui::event::KeyEvent;
use ratatui::crossterm::event::KeyCode;

use crate::InputEvent;

pub fn keyboard_input_system(
    mut events: EventReader<KeyEvent>,
    mut app_exit: EventWriter<AppExit>,
    mut input_event: EventWriter<InputEvent>,
) {
    for event in events.read() {
        match event.code {
            KeyCode::End => {
                app_exit.send_default();
            }
            KeyCode::Tab => {
                input_event.send(InputEvent::NextTab);
            }
            _ => {}
        }
    }
}
