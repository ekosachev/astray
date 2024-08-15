use std::time::Duration;

use bevy::app::ScheduleRunnerPlugin;
use bevy::log::tracing_subscriber::Layer;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use bevy::utils::tracing::Subscriber;
use bevy_ratatui::error::exit_on_error;
use bevy_ratatui::RatatuiPlugins;
use ratatui::widgets::canvas::Context;

use ui::system::body_list::BodyList;

use crate::components::general::Position;
use crate::systems::body_list_system::body_list_system;
use crate::systems::keyboard_input_system::keyboard_input_system;
use crate::systems::orbit_update_system::update_orbits;
use crate::systems::system_display_system::system_display_system;
use crate::systems::system_generator_system::generate_star_system;
use crate::systems::tab_system::tab_system;
use crate::systems::ui_system::ui_system;
use crate::ui::system::system_display::SystemDisplay;
use crate::ui::tab_menu::TabMenu;

mod components;
mod consts;
mod systems;
mod ui;

const FRAMERATE: f32 = 1. / 60.;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, States)]
enum Tab {
    System(SystemTabMode),
    Science,
    Colonies,
    ShipComponents,
}

impl Default for Tab {
    fn default() -> Self {
        Tab::System(SystemTabMode::default())
    }
}

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum SystemTabMode {
    #[default]
    Idle,
    SelectingBody,
    Map,
}

impl Tab {
    pub fn next(&self) -> Self {
        match self {
            Tab::System(_) => Tab::Science,
            Tab::Science => Tab::Colonies,
            Tab::Colonies => Tab::ShipComponents,
            Tab::ShipComponents => Tab::System(SystemTabMode::Idle),
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Tab::System(_) => Tab::ShipComponents,
            Tab::Science => Tab::System(SystemTabMode::Idle),
            Tab::Colonies => Tab::Science,
            Tab::ShipComponents => Tab::Colonies,
        }
    }
}

#[derive(Event)]
pub enum InputEvent {
    NextTab,
    PrevTab,
    StarSelectionInBodyList,
    BodyListUp,
    BodyListDown,
    BodyListFinishSelection,
    EnterMapNavMode,
    MapNavUp,
    MapNavDown,
    MapNavLeft,
    MapNavRight,
    MapNavZoomIn,
    MapNavZoomOut,
    MapNavFinish,
}

#[derive(Event)]
pub enum InGameEvent<'a> {
    RenderSystem(Context<'a>)
}

struct CustomLayer;

impl<S: Subscriber> Layer<S> for CustomLayer {
    fn on_event(
        &self,
        event: &bevy::utils::tracing::Event<'_>,
        _ctx: bevy::log::tracing_subscriber::layer::Context<'_, S>,
    ) {
        println!("Got event!");
        println!("  level={:?}", event.metadata().level());
        println!("  target={:?}", event.metadata().target());
        println!("  name={:?}", event.metadata().name());
    }
}

#[derive(Resource)]
pub struct CurrentSystem(Option<Entity>);

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
    let tab_menu = TabMenu {
        tab_titles: vec![
            "System".to_string(),
            "Science".to_string(),
            "Colonies".to_string(),
            "Ship Components".to_string(),
        ],
        selected_tab: 0,
    };
    app.insert_resource(tab_menu);

    let body_list = BodyList::default();
    app.insert_resource(body_list);
    app.insert_resource(CurrentSystem(None));

    app.insert_resource(SystemDisplay {
        translation: Position(0.0, 0.0),
        zoom: 10.0,
        is_focused: false,
    });
    // --- SYSTEMS ---
    app.add_systems(Update, ui_system.pipe(exit_on_error));
    app.add_systems(PreUpdate, keyboard_input_system);
    app.add_systems(Update, tab_system);
    app.add_systems(PreStartup, generate_star_system);
    app.add_systems(Update, body_list_system);
    app.add_systems(Update, update_orbits);
    app.add_systems(Update, system_display_system);
    // --- MISC ---
    app.init_state::<Tab>();
    app.add_event::<InputEvent>();
    app.run();
}
