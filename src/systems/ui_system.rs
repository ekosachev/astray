use bevy::prelude::{Query, Res, ResMut, State, With};
use bevy_ratatui::terminal::RatatuiContext;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::Line;

use crate::{CurrentSystem, Tab, ui::{
    system::body_list::{BodyList, render_body_list},
    tab_menu::{render_tab_menu, TabMenu},
}};
use crate::components::general::{BelongsToSolarSystem, Name, Orbit, Position, Radius,
                                 Renderable, Satellites};
use crate::components::planet::Planet;
use crate::components::star::Star;
use crate::ui::system::system_display::{render_system_display, SystemDisplay};

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn ui_system(
    mut context: ResMut<RatatuiContext>,
    state: Res<State<Tab>>,
    tab_menu: Res<TabMenu>,
    mut body_list: ResMut<BodyList>,
    system_display: Res<SystemDisplay>,
    maybe_system: Res<CurrentSystem>,
    stars: Query<(&Position, &Radius, &Renderable, &Satellites, &Name, &BelongsToSolarSystem),
        With<Star>>,
    planets: Query<(&Orbit, &Position, &Radius, &Renderable, &Name, &BelongsToSolarSystem), With<Planet>>
) -> color_eyre::Result<()> {
    context.draw(|frame| {
        let area = frame.size();
        let primary_chunks = Layout::new(
            Direction::Vertical,
            vec![Constraint::Length(2), Constraint::Fill(1)],
        )
        .split(area);
        render_tab_menu(frame, primary_chunks[0], &tab_menu);
        match state.get() {
            Tab::System(_) => {
                let secondary_chunks = Layout::new(
                    Direction::Horizontal,
                    vec![Constraint::Percentage(10), Constraint::Percentage(90)],
                )
                .split(primary_chunks[1]);
                render_body_list(frame, secondary_chunks[0], &mut body_list);

                let tertiary_chunks = Layout::new(
                    Direction::Vertical,
                    vec![Constraint::Percentage(80), Constraint::Percentage(20)],
                ).split(secondary_chunks[1]);


                if let Some(system) = maybe_system.0 {
                    if let Some(star_selected) = stars.iter()
                        .find(|(_, _, _, _, _, b)| {
                            b.0 == system
                        })
                    {
                        render_system_display(
                            frame,
                            tertiary_chunks[0],
                            &system_display,
                            (star_selected.0, star_selected.1, star_selected.2, star_selected.4),
                            planets.iter()
                                .filter(|planet| { planet.5.0 == system })
                                .map(|planet| {
                                    (planet.0, planet.1, planet.2, planet.3, planet.4)
                                })
                                .collect::<Vec<(&Orbit, &Position, &Radius, &Renderable, &Name)>>()
                                .as_slice(),
                        );
                    } else {
                        frame.render_widget(Line::from(
                            format!("No star; System: {:?}, Total stars: {:?}", system, stars
                                .iter().count())
                        ), tertiary_chunks[0])
                    }
                } else {
                    frame.render_widget(Line::from("No system"), tertiary_chunks[0])
                }
            }
            Tab::Colonies => frame.render_widget(Line::from("Colonies"), primary_chunks[1]),
            Tab::ShipComponents => frame.render_widget(
                Line::from("Ship Components"),
                primary_chunks[1],
            ),
            Tab::Science => frame.render_widget(Line::from("Science"), primary_chunks[1]),
        }
    })?;

    Ok(())
}
