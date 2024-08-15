use bevy::prelude::{AppExit, EventReader, EventWriter, Res, State};
use bevy_ratatui::event::KeyEvent;
use ratatui::crossterm::event::{KeyCode, KeyEventKind};

use crate::{InputEvent, SystemTabMode, Tab};

pub fn keyboard_input_system(
    mut events: EventReader<KeyEvent>,
    mut app_exit: EventWriter<AppExit>,
    mut input_event: EventWriter<InputEvent>,
    app_state: Res<State<Tab>>,
) {
    for event in events.read() {
        if event.kind != KeyEventKind::Press {
            continue;
        }
        match (event.code, app_state.get()) {
            (KeyCode::End, _) => {
                app_exit.send_default();
            }

            (KeyCode::Tab, Tab::System(SystemTabMode::Idle))
            | (KeyCode::Tab, Tab::Science)
            | (KeyCode::Tab, Tab::Colonies)
            | (KeyCode::Tab, Tab::ShipComponents) => {
                input_event.send(InputEvent::NextTab);
            }

            (KeyCode::BackTab, Tab::System(SystemTabMode::Idle))
            | (KeyCode::BackTab, Tab::Science)
            | (KeyCode::BackTab, Tab::Colonies)
            | (KeyCode::BackTab, Tab::ShipComponents) => {
                input_event.send(InputEvent::PrevTab);
            }

            (KeyCode::Char('q'), Tab::System(SystemTabMode::Idle)) => {
                input_event.send(InputEvent::StarSelectionInBodyList);
            }
            (KeyCode::Char('e'), Tab::System(SystemTabMode::Idle)) => {
                input_event.send(InputEvent::EnterMapNavMode);
            }

            (KeyCode::Char('e'), Tab::System(SystemTabMode::Map)) => {
                input_event.send(InputEvent::MapNavFinish);
            }

            (KeyCode::Char('w'), Tab::System(SystemTabMode::Map)) => {
                input_event.send(InputEvent::MapNavUp);
            }

            (KeyCode::Char('s'), Tab::System(SystemTabMode::Map)) => {
                input_event.send(InputEvent::MapNavDown);
            }

            (KeyCode::Char('a'), Tab::System(SystemTabMode::Map)) => {
                input_event.send(InputEvent::MapNavLeft);
            }

            (KeyCode::Char('d'), Tab::System(SystemTabMode::Map)) => {
                input_event.send(InputEvent::MapNavRight);
            }

            (KeyCode::Char('r'), Tab::System(SystemTabMode::Map)) => {
                input_event.send(InputEvent::MapNavZoomIn);
            }

            (KeyCode::Char('f'), Tab::System(SystemTabMode::Map)) => {
                input_event.send(InputEvent::MapNavZoomOut);
            }
            
            (KeyCode::Up, Tab::System(SystemTabMode::SelectingBody)) => {
                input_event.send(InputEvent::BodyListUp);
            }

            (KeyCode::Down, Tab::System(SystemTabMode::SelectingBody)) => {
                input_event.send(InputEvent::BodyListDown);
            }

            (KeyCode::Enter, Tab::System(SystemTabMode::SelectingBody)) => {
                input_event.send(InputEvent::BodyListFinishSelection);
            }

            _ => {}
        }
    }
}
