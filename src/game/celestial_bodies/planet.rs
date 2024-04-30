use std::ops::RangeInclusive;

use ordered_float::OrderedFloat;
use rand::distributions::Distribution;
use rand_distr::num_traits::ToPrimitive;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};

use crate::game::celestial_bodies::{CanOrbit, CelestialBody, CelestialBodyType, Displayable};
use crate::game::celestial_bodies::solar_system::SolarSystem;
use crate::game::helpers::{consts, orbit_dynamics};
use crate::game::helpers::astrophysics::calculate_habitable_zone_from_luminosity;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Planet {
    name: String,
    mass: OrderedFloat<f32>,
    radius: OrderedFloat<f32>,
    orbit_radius: OrderedFloat<f32>,
    orbit_period: OrderedFloat<f32>,
    habitable_zone: RangeInclusive<OrderedFloat<f32>>,
}

impl CelestialBody for Planet {
    type HostType = SolarSystem;

    fn get_type(&self) -> CelestialBodyType {
        CelestialBodyType::Planet
    }

    fn get_mass(&self) -> f32 {
        self.mass.to_f32().unwrap()
    }

    fn get_radius(&self) -> f32 {
        self.radius.to_f32().unwrap()
    }

    fn generate(host: SolarSystem) -> Self {
        let rng = rand::thread_rng();
        let n = host.get_n_planets() + 1;
        let letter: char = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().nth(n - 1).unwrap();

        let mass = rand_distr::Normal::new(
            1.5,
            0.7,
        ).unwrap().sample(&mut rng.clone()) * consts::EARTH_M_KG;
        
        let density = rand_distr::Normal::new(
            5500.0,
            750.0,
        ).unwrap().sample(&mut rng.clone());
        
        let volume = mass / density;
        let radius = (volume / ((4.0 / 3.0) * std::f32::consts::PI)).cbrt();

        let orbit_radius = if n == 1 {
            // This is the innermost planet
            let inner_limit = host.get_inner_limit();

            inner_limit + rand_distr::Normal::new(
                0.4,
                0.2,
            ).unwrap().sample(&mut rng.clone()) * consts::AU_M
        } else {
            host.get_nth_orbit_radius(n as u32)
        };

        let orbital_velocity = orbit_dynamics::calculate_orbital_velocity(
            host.get_star_mass(),
            orbit_radius,
        );

        let orbit_period = (std::f32::consts::TAU * orbit_radius) / orbital_velocity;
        
        let mut name = host.get_name();
        name.push(' ');
        name.push(letter);

        let habitable_zone = calculate_habitable_zone_from_luminosity(
            host.get_star().get_luminosity()
        );

        Self {
            name,
            mass: OrderedFloat(mass),
            radius: OrderedFloat(radius),
            orbit_radius: OrderedFloat(orbit_radius.clone()),
            orbit_period: OrderedFloat(orbit_period),
            habitable_zone: RangeInclusive::new(
                OrderedFloat(*habitable_zone.start()),
                OrderedFloat(*habitable_zone.end()),
            )
        }
    }
}

impl CanOrbit for Planet {
    type HostType = SolarSystem;

    fn get_orbit_radius(&self) -> f32 {
        self.orbit_radius.to_f32().unwrap()
    }

    fn get_orbit_period(&self) -> f32 {
        self.orbit_period.to_f32().unwrap()
    }
}

impl Displayable for Planet {
    fn get_properties(&self) -> Vec<Vec<String>> {
        vec![
            vec![
                String::from("Mass"),
                format!("{:.3E} kg", self.mass),
                format!("{:.3} earth masses", self.mass / consts::EARTH_M_KG),
            ],
            vec![
                String::from("Radius"),
                format!("{:.3E} m", self.radius),
                format!("{:.3} earth radii", self.radius / consts::EARTH_R_M),
            ],
            vec![
                String::from("Orbit radius"),
                format!("{:.3E} m", self.orbit_radius),
                format!("{:.3} AU", self.orbit_radius / consts::AU_M),
            ],
            vec![
                String::from("Orbital period"),
                format!("{:.3E} s", self.orbit_period),
                format!("{:.3} yrs", self.orbit_period / consts::S_YR as f32),
            ],
            
        ]
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_menu_color(&self) -> Color {
        if self.is_inside_habitable_zone() {
            Color::LightGreen
        } else if self.habitable_zone.start() > &self.orbit_radius {
            Color::LightYellow
        } else {
            Color::LightRed
        }
    }
}

impl Planet {
    pub fn is_inside_habitable_zone(&self) -> bool {
        self.habitable_zone.contains(&self.orbit_radius)
    }
}