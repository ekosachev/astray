use bevy::prelude::{EventReader, NextState, ResMut};

use crate::{InputEvent, SystemTabMode, Tab};
use crate::ui::system::system_display::SystemDisplay;

pub fn system_display_system(
    mut system_display: ResMut<SystemDisplay>,
    mut events: EventReader<InputEvent>,
    mut tab: ResMut<NextState<Tab>>,
) {
    for event in events.read() {
        match event {
            InputEvent::EnterMapNavMode => {
                tab.set(Tab::System(SystemTabMode::Map));
                system_display.is_focused = true;
            }

            InputEvent::MapNavFinish => {
                tab.set(Tab::System(SystemTabMode::Idle));
                system_display.is_focused = false;
            }

            InputEvent::MapNavUp => {
                system_display.translation.1 += system_display.zoom;
            }

            InputEvent::MapNavDown => {
                system_display.translation.1 -= system_display.zoom;
            }

            InputEvent::MapNavLeft => {
                system_display.translation.0 -= system_display.zoom;
            }

            InputEvent::MapNavRight => {
                system_display.translation.0 += system_display.zoom;
            }

            InputEvent::MapNavZoomIn => {
                system_display.zoom = (system_display.zoom / 1.2).clamp(0.01, 100.0);
            }

            InputEvent::MapNavZoomOut => {
                system_display.zoom = (system_display.zoom * 1.2).clamp(0.01, 100.0);
            }

            _ => {}
        }
    }
}
