use std::fs;

use bevy::prelude::{Bundle, Component};
use rand::distributions::Distribution;
use rand::prelude::SliceRandom;

use crate::components::general::{Mass, Name, Radius, Satellites, Temperature};
use crate::consts::physics::conversion_ratios::{
    SOLAR_LUMINOSITY_TO_WHATS, SOLAR_MASS_TO_KG, SOLAR_RADII_TO_M,
};

pub enum StarClass {
    O,
    B,
    A,
    F,
    G,
    K,
    M,
}
#[derive(Component)]
pub struct Star {
    pub luminosity: f32,
    pub star_class: StarClass,
}

#[derive(Bundle)]
pub struct StarBundle {
    pub star: Star,
    pub name: Name,
    pub mass: Mass,
    pub radius: Radius,
    pub temperature: Temperature,
    pub satellites: Satellites,
}

fn calculate_luminosity(mass: f32) -> f32 {
    let solar_mass = mass / SOLAR_MASS_TO_KG;
    let solar_luminosity = if solar_mass <= 0.43 {
        0.23 * solar_mass.powf(2.3)
    } else if solar_mass <= 2.0 {
        solar_mass.powi(4)
    } else {
        1.4 * solar_mass.powf(3.5)
    };

    solar_luminosity * SOLAR_LUMINOSITY_TO_WHATS
}

fn calculate_radius(mass: f32) -> f32 {
    let solar_mass = mass / SOLAR_MASS_TO_KG;
    let radius = if solar_mass <= 1.0 {
        solar_mass.powf(0.8)
    } else {
        solar_mass.powf(0.57)
    };

    radius * SOLAR_RADII_TO_M
}

fn calculate_temperature(luminosity: f32, radius: f32) -> f32 {
    let solar_luminosity = luminosity / SOLAR_LUMINOSITY_TO_WHATS;
    let solar_radius = radius / SOLAR_RADII_TO_M;

    ((solar_luminosity / solar_radius.powi(2)).powf(0.25)) * 5776.0
}

fn determine_star_class(temperature: f32) -> StarClass {
    match temperature as i32 {
        3700..=5200 => StarClass::K,
        5201..=6000 => StarClass::G,
        6001..=7500 => StarClass::F,
        7501..=10000 => StarClass::A,
        10001..=33000 => StarClass::B,
        33001..=i32::MAX => StarClass::O,

        _ => StarClass::M,
    }
}

impl StarBundle {
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();
        let mass = rand_distr::Normal::new(1.2, 0.2).unwrap().sample(&mut rng) * SOLAR_MASS_TO_KG;
        let luminosity = calculate_luminosity(mass);
        let radius = calculate_radius(mass);
        let temperature = calculate_temperature(luminosity, radius);
        let star_class = determine_star_class(temperature);

        let star_names: Vec<String> = fs::read_to_string("./assets/namelists/star_namelist.txt")
            .unwrap()
            .split("\r\n")
            .map(|s| s.to_string())
            .collect();

        Self {
            star: Star {
                luminosity,
                star_class,
            },
            name: Name(star_names.choose(&mut rng).unwrap().clone()),
            mass: Mass(mass),
            radius: Radius(radius),
            temperature: Temperature(temperature),
            satellites: Satellites(Vec::new()),
        }
    }
}
