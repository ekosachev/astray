use bevy::prelude::{EventReader, NextState, Query, Res, ResMut, With};

use crate::{CurrentSystem, InputEvent, SystemTabMode, Tab};
use crate::components::general::{BelongsToSolarSystem, Name};
use crate::components::planet::Planet;
use crate::components::star::Star;
use crate::ui::system::body_list::BodyList;

pub fn body_list_system(
    mut body_list: ResMut<BodyList>,
    mut events: EventReader<InputEvent>,
    mut tab: ResMut<NextState<Tab>>,
    maybe_system: Res<CurrentSystem>,
    star_names: Query<(&Name, &BelongsToSolarSystem), With<Star>>,
    planet_names: Query<(&Name, &BelongsToSolarSystem), With<Planet>>,
) {
    if let Some(system) = maybe_system.0 {
        body_list.items.clear();

        let maybe_star = star_names.iter().find(
            |(_, s)| {
                s.0 == system
            }
        );

        if let Some(star) = maybe_star {
            body_list.items.push(star.0.clone().0);

            let planets = planet_names.iter().filter(
                |(_, p)| {
                    p.0 == system
                }
            ).map(
                |(n, _)| {
                    n.0.clone()
                }
            ).collect::<Vec<String>>();

            body_list.items.extend(planets);
        } else {
            body_list.items.push(String::from("No star found"))
        }
    } else {
        body_list.items.push(String::from("No system set"))
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
