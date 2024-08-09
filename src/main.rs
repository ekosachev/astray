use std::time::Duration;

use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use bevy_ratatui::error::exit_on_error;
use bevy_ratatui::RatatuiPlugins;

use crate::systems::keyboard_input_system::keyboard_input_system;
use crate::systems::ui_system::ui_system;

mod systems;
mod ui;

const FRAMERATE: f32 = 1. / 60.;

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash, Debug, States)]
enum Tab {
    #[default]
    System,
    Science,
    Colonies,
    ShipComponents,
}

#[derive(Event)]
pub enum InputEvent {
    NextTab,
    PrevTab,
}

fn main() {
    let mut app = App::new();
    // --- PLUGINS ---
    app.add_plugins(RatatuiPlugins::default());
    app.add_plugins(
        MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f32(
            FRAMERATE,
        ))),
    );
    app.add_plugins(StatesPlugin);
    // --- RESOURCES ---
    // --- SYSTEMS ---
    app.add_systems(Update, ui_system.pipe(exit_on_error));
    app.add_systems(PreUpdate, keyboard_input_system);
    // --- MISC ---
    app.init_state::<Tab>();
    app.add_event::<InputEvent>();
    app.run();
}
