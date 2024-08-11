use std::f32::consts::{PI, TAU};

use bevy::prelude::Bundle;
use rand::distributions::Distribution;
use rand::Rng;

use crate::components::general::{Mass, Name, Orbit, Radius};
use crate::components::star::StarBundle;
use crate::consts::physics::constants::G;
use crate::consts::physics::conversion_ratios::EARTH_MASS_TO_KG;

#[derive(Bundle, Clone)]
pub struct PlanetBundle {
    pub name: Name,
    pub mass: Mass,
    pub radius: Radius,
    pub orbit: Orbit,
}

fn roche_limit(primary_mass: f32, secondary_mass: f32) -> f32 {
    secondary_mass * (2. * primary_mass / secondary_mass).cbrt()
}

fn sphere_of_influence(primary_mass: f32, secondary_mass: f32) -> f32 {
    (secondary_mass / primary_mass).powf(0.4)
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
        let sphere_of_influence = sphere_of_influence(planet.mass.0, new_mass);

        orbit_diff < sphere_of_influence
    })
}

impl PlanetBundle {
    pub fn generate(star: &StarBundle, existing_planets: Vec<Self>) -> Self {
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
        let roche_limit = roche_limit(star.mass.0, mass);
        let sphere_of_influence = sphere_of_influence(star.mass.0, mass);

        let mut orbit_radius = rand::thread_rng().gen_range(roche_limit..=sphere_of_influence);

        // While orbit conflicts exist, generate a new orbit radius value
        // Orbit conflicts appear, when one planet's orbit crosses other
        // planet's sphere of influence
        // TODO: Make new planet become a satellite of an existing one in case of an orbit conflict
        while has_orbit_conflicts(existing_planets.clone(), orbit_radius, mass) {
            orbit_radius = rand::thread_rng().gen_range(roche_limit..=sphere_of_influence);
        }

        // Calculate orbital velocity based on centripetal acceleration
        let orbital_velocity = orbital_velocity(star.mass.0, orbit_radius);

        // Calculate orbital period (time that it takes for a planet to
        // complete a full orbit)
        let orbital_period = TAU * orbit_radius / orbital_velocity;

        Self {
            name: Name(star.name.0.clone() + " " + &*letter.to_string()),
            mass: Mass(mass),
            radius: Radius(radius),
            orbit: Orbit {
                radius: orbit_radius,
                period: orbital_period,
                position: rand::thread_rng().gen_range(0.0..TAU),
            },
        }
    }
}
