use bevy::prelude::Resource;
use color_eyre::owo_colors::OwoColorize;
use ratatui::Frame;
use ratatui::prelude::{Color, Modifier, Rect, Span, Style};
use ratatui::widgets::{Block, Borders};
use ratatui::widgets::canvas::{Canvas, Circle};

use crate::components::general::{Name, Orbit, Position, Radius, Renderable};
use crate::consts::physics::conversion_ratios::AU_TO_M;

#[derive(Resource)]
pub struct SystemDisplay {
    pub translation: Position,
    pub zoom: f32,
    pub is_focused: bool,
}

pub fn render_system_display(
    frame: &mut Frame,
    area: Rect,
    data: &SystemDisplay,
    star: (&Position, &Radius, &Renderable, &Name),
    planets: &[(&Orbit, &Position, &Radius, &Renderable, &Name)],
) {
    let ar = (area.width as f32) / (area.height as f32) / 2.;
    let height = [
        ((-15. + data.translation.1) / ar * data.zoom) as f64,
        ((15. + data.translation.1) / ar * data.zoom) as f64,
    ];
    let width = [
        ((-15. + data.translation.0) * data.zoom) as f64,
        ((15. + data.translation.0) * data.zoom) as f64,
    ];

    let mut display = Canvas::default()
        .block(Block::default().borders(Borders::BOTTOM))
        .x_bounds(width)
        .y_bounds(height)
        .paint(|ctx| {
            ctx.draw(
                &Circle {
                    x: star.0.0 as f64,
                    y: star.0.1 as f64,
                    radius: (star.1.0 / AU_TO_M).clamp(0.05, 100.0) as f64,
                    color: star.2.0,
                }
            );
            ctx.print(
                (star.0.0 + 0.2 * data.zoom) as f64,
                (star.0.1 + 0.2 * data.zoom) as f64,
                Span::styled(
                    star.3.0.clone(),
                    Style::default().fg(star.2.0).add_modifier(Modifier::BOLD),
                ),
            );

            planets.iter().for_each(
                |(orbit, pos, r, renderable, name)| {
                    // Draw orbit
                    ctx.draw(
                        &Circle {
                            x: star.0.0 as f64,
                            y: star.0.1 as f64,
                            radius: (orbit.radius / AU_TO_M) as f64,
                            color: Color::DarkGray,
                        }
                    );
                    // Draw the planet
                    ctx.draw(
                        &Circle {
                            x: ((star.0.0 + pos.0) / AU_TO_M) as f64,
                            y: ((star.0.1 + pos.1) / AU_TO_M) as f64,
                            radius: (r.0 / AU_TO_M).clamp(0.05, 100.0) as f64,
                            color: renderable.0,
                        }
                    );
                    ctx.print(
                        ((star.0.0 + pos.0) / AU_TO_M + 0.2 * data.zoom) as f64,
                        ((star.0.1 + pos.1) / AU_TO_M + 0.2 * data.zoom) as f64,
                        Span::styled(
                            name.0.clone(),
                            Style::default().fg(renderable.0),
                        ),
                    );
                }
            );
        });

    if data.is_focused {
        display = display.background_color(Color::Rgb(50, 50, 50));
    }
    frame.render_widget(display, area);
    frame.render_widget(
        Span::from(
            format!(
                "X: {}, Y: {}, ZOOM: {:.2}",
                data.translation.0,
                data.translation.1,
                data.zoom
            )
        ),
        area,
    );
}