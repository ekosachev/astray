use bevy::prelude::{EventReader, NextState, Query, Res, ResMut, With};

use crate::{CurrentSystem, InputEvent, SystemTabMode, Tab};
use crate::components::general::{Name, Satellites};
use crate::components::planet::Planet;
use crate::components::star::Star;
use crate::ui::system::body_list::BodyList;

pub fn body_list_system(
    mut body_list: ResMut<BodyList>,
    mut events: EventReader<InputEvent>,
    mut tab: ResMut<NextState<Tab>>,
    maybe_system: Res<CurrentSystem>,
    star_names: Query<&Name, With<Star>>,
    satellites: Query<&Satellites, With<Star>>,
    planet_names: Query<&Name, With<Planet>>,
) {
    if let Some(system) = maybe_system.0 {
        let star_name = star_names.get(system).unwrap().0.clone();
        let planet_names: Vec<String> = satellites
            .get(system)
            .unwrap()
            .0
            .iter()
            .map(|s| planet_names.get(*s).unwrap().0.clone())
            .collect();

        body_list.items.clear();
        body_list.items.push(star_name);
        body_list.items.extend(planet_names);
    }

    for event in events.read() {
        match event {
            InputEvent::StarSelectionInBodyList => {
                body_list.is_focused = true;
                if body_list.list_state.selected().is_none() {
                    body_list.list_state.select(Some(0));
                }
                tab.set(Tab::System(SystemTabMode::SelectingBody));
            }

            InputEvent::BodyListUp => {
                let currently_selected = body_list.list_state.selected().unwrap();
                let total = body_list.items.len();
                body_list
                    .list_state
                    .select(Some((currently_selected + total - 1) % total))
            }

            InputEvent::BodyListDown => {
                let currently_selected = body_list.list_state.selected().unwrap();
                let total = body_list.items.len();
                body_list
                    .list_state
                    .select(Some((currently_selected + 1) % total))
            }

            InputEvent::BodyListFinishSelection => {
                body_list.is_focused = false;
                tab.set(Tab::System(SystemTabMode::Idle));
            }

            _ => {}
        }
    }
}
