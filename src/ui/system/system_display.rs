use bevy::math::Vec2;
use bevy::prelude::Resource;
use color_eyre::owo_colors::OwoColorize;
use ratatui::Frame;
use ratatui::prelude::{Color, Modifier, Rect, Span, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::widgets::canvas::{Canvas, Circle, Line};

use crate::components::general::{Mass, Name, Orbit, Position, Radius, Renderable};
use crate::consts::physics::conversion_ratios::AU_TO_M;

#[derive(Resource)]
pub struct SystemDisplay {
    pub translation: Position,
    pub zoom: f32,
    pub is_focused: bool,
}

const GRADIENT_WEAK: (u8, u8, u8) = (69, 202, 255);
const GRADIENT_STRONG: (u8, u8, u8) = (255, 27, 107);

fn gravity_gradient(min: usize, max: usize, p: usize) -> Color {
    let k = (p - min) as f32 / (max - min) as f32;
    let (min_r, min_g, min_b) = (
        GRADIENT_WEAK.0/*.min(GRADIENT_STRONG.0)*/,
        GRADIENT_WEAK.1/*.min(GRADIENT_STRONG.1)*/,
        GRADIENT_WEAK.2/*.min(GRADIENT_STRONG.2)*/
    );
    let (max_r, max_g, max_b) = (
        GRADIENT_STRONG.0/*.max(GRADIENT_STRONG.0)*/,
        GRADIENT_STRONG.1/*.max(GRADIENT_STRONG.1)*/,
        GRADIENT_STRONG.2/*.max(GRADIENT_STRONG.2)*/
    );
    Color::Rgb(
        (max_r as f32 * k + min_r as f32 * (1. - k)) as u8,
        (max_g as f32 * k + min_g as f32 * (1. - k)) as u8,
        (max_b as f32 * k + min_b as f32 * (1. - k)) as u8,
    )
}

fn lerp(min: f32, max: f32, n_min: f32, n_max: f32, value: f32) -> f32 {
    let k = (value - min) / (max - min);
    n_min + (n_max - n_min) * k
}

pub fn render_system_display(
    frame: &mut Frame,
    area: Rect,
    data: &SystemDisplay,
    star: (&Position, &Radius, &Renderable, &Name, &Mass),
    planets: &[(&Orbit, &Position, &Radius, &Renderable, &Name, &Mass)],
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

    // Calculate the hashmap (x, y) => potential
    let mut point_masses: Vec<((f64, f64), f32)> = Vec::new();
    point_masses.push(((star.0.0 as f64, star.0.1 as f64), star.4.0));

    point_masses.extend(
        planets.iter().map(|(_, pos, _, _, _, mass)| {
            ((pos.0 as f64, pos.1 as f64), mass.0)
        })
    );

    let mut potential_map: Vec<((f64, f64), usize)> = Vec::new();
    let step = ((data.zoom / 5.0) as usize).max(1);

    let points_width = (((width[0]) as i32)..=((width[1]) as i32)).step_by(step).count();
    let points_height = (((height[0]) as i32)..=((height[1]) as i32)).step_by(step)
        .count();

    for y in (((height[0]) as i32)..=((height[1]) as i32)).step_by(step) {
        for x in (((width[0]) as i32)..=((width[1]) as i32)).step_by
        (step) {
            potential_map.push(
                ((x as f64 * AU_TO_M as f64, y as f64 * AU_TO_M as f64),
                 point_masses.iter().map(|((body_x, body_y), mass)| {
                     let measuring_point = Vec2::new(x as f32 * AU_TO_M, y as
                         f32
                         * AU_TO_M);
                     let body_position = Vec2::new(*body_x as f32, *body_y as f32);
                     let distance = measuring_point - body_position;
                     let r = distance.normalize_or_zero();
                     r * (mass / distance.length().powi(2))
                 }).sum::<Vec2>().length() as usize)
            )
        }
    }

    // let AMOUNT_OF_EQUIPOTENTIALS = (data.zoom * 2.0).clamp(10.0, 50.0) as usize;
    let AMOUNT_OF_EQUIPOTENTIALS = 10;
    let max_potential = potential_map.iter().max_by_key(|(_, v)| v).unwrap().1;
    let min_potential = potential_map.iter().min_by_key(|(_, v)| v).unwrap().1;
    let mut thresholds: Vec<usize> = vec![0; AMOUNT_OF_EQUIPOTENTIALS];

    let min_x = (min_potential as f32).sqrt();
    // let min_x = 0.0f32;
    let max_x = (max_potential as f32).sqrt();
    let diff = max_x / AMOUNT_OF_EQUIPOTENTIALS as f32;
    for i in 1..=AMOUNT_OF_EQUIPOTENTIALS {
        thresholds[i - 1] = (min_x + (i as f32 * diff)).powi(2) as usize;
    }

    // Calculate equipotential lines using marching squares algorithm
    const LINE: Vec<((f64, f64), (f64, f64))> = Vec::new();
    let mut equipotentials: Vec<Vec<((f64, f64), (f64, f64))>> = vec![LINE; AMOUNT_OF_EQUIPOTENTIALS];

    for n in 0..AMOUNT_OF_EQUIPOTENTIALS {
        for i in 0..(points_width - 1) {
            for j in 0..(points_height - 1) {
                let points = [
                    potential_map[j * points_width + i],
                    potential_map[j * points_width + i + 1],
                    potential_map[(j + 1) * points_width + i + 1],
                    potential_map[j * points_width + i],
                ];
                let cell: [bool; 4] = [
                    points[0].1 > thresholds[n],
                    points[1].1 > thresholds[n],
                    points[2].1 > thresholds[n],
                    points[3].1 > thresholds[n],
                ];
                let midpoints = [
                    (
                        lerp(
                            points[0].1 as f32,
                            points[1].1 as f32,
                            points[0].0.0 as f32,
                            points[1].0.0 as f32,
                            thresholds[n] as f32,
                        ) as f64,
                        points[0].0.1,
                    ),
                    (
                        points[1].0.0,
                        lerp(
                            points[1].1 as f32,
                            points[2].1 as f32,
                            points[1].0.1 as f32,
                            points[2].0.1 as f32,
                            thresholds[n] as f32,
                        ) as f64,
                    ),
                    (
                        lerp(
                            points[2].1 as f32,
                            points[3].1 as f32,
                            points[2].0.0 as f32,
                            points[3].0.0 as f32,
                            thresholds[n] as f32,
                        ) as f64,
                        points[2].0.1,
                    ),
                    (
                        points[3].0.0,
                        lerp(
                            points[3].1 as f32,
                            points[0].1 as f32,
                            points[3].0.1 as f32,
                            points[0].0.1 as f32,
                            thresholds[n] as f32,
                        ) as f64,
                    ),
                ];
                equipotentials[n].extend::<Vec<((f64, f64), (f64, f64))>>(
                    match cell {
                        [false, false, false, false] => vec![],
                        [false, false, false, true] => vec![(midpoints[2], midpoints[3])],
                        [false, false, true, false] => vec![(midpoints[1], midpoints[2])],
                        [false, false, true, true] => vec![(midpoints[1], midpoints[3])],
                        [false, true, false, false] => vec![(midpoints[0], midpoints[1])],
                        [false, true, false, true] => vec![
                            (midpoints[0], midpoints[3]),
                            (midpoints[1], midpoints[2]),
                        ],
                        [false, true, true, false] => vec![(midpoints[0], midpoints[2])],
                        [false, true, true, true] => vec![(midpoints[0], midpoints[3])],
                        [true, false, false, false] => vec![(midpoints[0], midpoints[3])],
                        [true, false, false, true] => vec![(midpoints[0], midpoints[2])],
                        [true, false, true, false] => vec![
                            (midpoints[0], midpoints[1]),
                            (midpoints[2], midpoints[3]),
                        ],
                        [true, false, true, true] => vec![(midpoints[0], midpoints[1])],
                        [true, true, false, false] => vec![(midpoints[1], midpoints[3])],
                        [true, true, false, true] => vec![(midpoints[1], midpoints[2])],
                        [true, true, true, false] => vec![(midpoints[2], midpoints[3])],
                        [true, true, true, true] => vec![],
                    }
                )
            }
        }
    }


    let mut display = Canvas::default()
        .block(Block::default().borders(Borders::BOTTOM))
        .x_bounds(width)
        .y_bounds(height)
        .paint(|ctx| {
            equipotentials.iter().enumerate().for_each(
                |(i, lines)| {
                    lines.iter().for_each(
                        |line_coords| {
                            ctx.draw(
                                &Line {
                                    x1: line_coords.0.0 / AU_TO_M as f64,
                                    y1: line_coords.0.1 / AU_TO_M as f64,
                                    x2: line_coords.1.0 / AU_TO_M as f64,
                                    y2: line_coords.1.1 / AU_TO_M as f64,
                                    color: gravity_gradient(
                                        1, AMOUNT_OF_EQUIPOTENTIALS, i + 1,
                                    ),
                                }
                            )
                        }
                    )
                }
            );

            ctx.layer();
            // Draw celestial objects
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
                |(orbit, pos, r, renderable, name, _)| {
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
        Paragraph::new(vec![
            ratatui::text::Line::styled(
                format!(
                    "X: {}, Y: {}, ZOOM: {:.2}",
                    data.translation.0,
                    data.translation.1,
                    data.zoom,
                ),
                Style::default().fg(Color::White),
            ),
            // ratatui::text::Line::styled(
            //     format!(
            //         "MIN_P: {}, MAX_P: {}, STEP: {}, DIFF: {}, THRESHOLDS: {:?}",
            //         min_potential,
            //         max_potential,
            //         step,
            //         diff,
            //         thresholds
            //     ),
            //     Style::default().fg(Color::White),
            // ),
            // ratatui::text::Line::styled(
            //     format!(
            //         "{:?}",
            //         equipotentials
            //     ),
            //     Style::default().fg(Color::White),
            // ),
        ]),
        area,
    );
}