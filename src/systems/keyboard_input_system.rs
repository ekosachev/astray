use bevy::prelude::{AppExit, EventReader, EventWriter};
use bevy_ratatui::event::KeyEvent;
use ratatui::crossterm::event::{KeyCode, KeyEventKind};

use crate::InputEvent;

pub fn keyboard_input_system(
    mut events: EventReader<KeyEvent>,
    mut app_exit: EventWriter<AppExit>,
    mut input_event: EventWriter<InputEvent>,
) {
    for event in events.read() {
        if event.kind != KeyEventKind::Press {
            continue
        }
        match event.code {
            KeyCode::End => {
                app_exit.send_default();
            }
            KeyCode::Tab => {
                input_event.send(InputEvent::NextTab);
            }
            KeyCode::BackTab => {
                input_event.send(InputEvent::PrevTab);
            }
            _ => {}
        }
    }
}
