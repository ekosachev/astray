use std::f32::consts::{PI, TAU};

use bevy::prelude::{Bundle, Component, Entity};
use rand::distributions::Distribution;
use rand::Rng;
use ratatui::prelude::Color;

use crate::components::general::{BelongsToSolarSystem, Mass, Name, Orbit, Position, Radius, Renderable};
use crate::components::star::StarBundle;
use crate::consts::physics::constants::G;
use crate::consts::physics::conversion_ratios::EARTH_MASS_TO_KG;

const MAX_ORBIT_GENERATION_TRIES: i32 = 100;

#[derive(Component, Clone)]
pub struct Planet {}

#[derive(Bundle, Clone)]
pub struct PlanetBundle {
    pub planet: Planet,
    pub position: Position,
    pub name: Name,
    pub mass: Mass,
    pub radius: Radius,
    pub orbit: Orbit,
    pub renderable: Renderable,
    pub system: BelongsToSolarSystem,
}

fn roche_limit(primary_mass: f32, primary_radius: f32, secondary_mass: f32) -> f32 {
    primary_radius * (2. * primary_mass / secondary_mass).cbrt()
}

fn sphere_of_influence(primary_mass: f32, primary_orbit_radius: f32, secondary_mass: f32) -> f32 {
    primary_orbit_radius * (secondary_mass / primary_mass).powf(0.4)
}

fn orbital_velocity(primary_mass: f32, orbit_radius: f32) -> f32 {
    (G * primary_mass / orbit_radius).sqrt()
}

/// Outputs `true` if there is an orbit conflict, `false` otherwise.
fn has_orbit_conflicts(
    existing_planets: Vec<PlanetBundle>,
    new_orbit_radius: f32,
    new_mass: f32,
) -> bool {
    existing_planets.iter().any(|planet| {
        let orbit_diff = (planet.orbit.radius - new_orbit_radius).abs();
        let sphere_of_influence = sphere_of_influence(planet.mass.0, planet.orbit.radius, new_mass);

        orbit_diff < sphere_of_influence
    })
}

impl PlanetBundle {
    pub fn generate(star: &StarBundle, existing_planets: Vec<Self>, system: Entity) -> Self {
        let mut rng = rand::thread_rng();
        let n = existing_planets.len() as usize;
        let letter: char = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().nth(n).unwrap();

        let mass = rand_distr::Normal::new(1.5, 0.7).unwrap().sample(&mut rng) * EARTH_MASS_TO_KG;

        let density = rand_distr::Normal::new(5500.0, 750.0)
            .unwrap()
            .sample(&mut rng);

        let volume = mass / density;
        let radius = (volume / ((4.0 / 3.0) * PI)).cbrt();

        // Generate orbit radius
        // Calculate star's roche limit and sphere of influence, these values
        // are used as a range for random generation
        let roche_limit = roche_limit(star.mass.0, star.radius.0, mass);
        let sphere_of_influence = 1000.0 * roche_limit; // TODO: replace with a better formula

        let mut orbit_radius = rand::thread_rng().gen_range(roche_limit..=sphere_of_influence);

        // While orbit conflicts exist, generate a new orbit radius value
        // Orbit conflicts appear, when one planet's orbit crosses other
        // planet's sphere of influence
        // TODO: Make new planet become a satellite of an existing one in case of an orbit conflict

        let mut orbit_generation_tries = 0;
        while has_orbit_conflicts(existing_planets.clone(), orbit_radius, mass)
            && (MAX_ORBIT_GENERATION_TRIES > orbit_generation_tries)
        {
            orbit_radius = rand::thread_rng().gen_range(roche_limit..=sphere_of_influence);
            orbit_generation_tries += 1;
        }

        // Calculate orbital velocity based on centripetal acceleration
        let orbital_velocity = orbital_velocity(star.mass.0, orbit_radius);

        // Calculate orbital period (time that it takes for a planet to
        // complete a full orbit)
        let orbital_period = TAU * orbit_radius / orbital_velocity;

        Self {
            planet: Planet {},
            position: Position(0.0, 0.0),
            name: Name(star.name.0.clone() + " " + &*letter.to_string()),
            mass: Mass(mass),
            radius: Radius(radius),
            orbit: Orbit {
                radius: orbit_radius,
                period: orbital_period,
                position: rand::thread_rng().gen_range(0.0..TAU),
            },
            renderable: Renderable(Color::Green),
            system: BelongsToSolarSystem(system),
        }
    }
}
