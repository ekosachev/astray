use rand::Rng;
use rand::distributions::Distribution;
use ratatui::prelude::Color;
use crate::game::celestial_bodies::{CanOrbit, CelestialBody, CelestialBodyType};
use crate::game::celestial_bodies::solar_system::SolarSystem;
use crate::game::celestial_bodies::star::Star;
use crate::game::helpers::{orbit_dynamics, astrophysics, consts};

#[derive(Clone)]
pub struct Planet {
    name: String,
    mass: f32,
    radius: f32,
    orbit_radius: f32,
    orbit_period: f32,
}

impl Planet {
    pub fn get_orbit_radius(&self) -> f32 {
        self.orbit_radius
    }
}

impl CelestialBody for Planet {
    type HostType = SolarSystem;
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_type(&self) -> CelestialBodyType {
        CelestialBodyType::Planet
    }

    fn get_mass(&self) -> f32 {
        self.mass
    }

    fn get_radius(&self) -> f32 {
        self.radius
    }

    // fn get_menu_color(&self) -> Color {
    //     todo!()
    // }

    fn generate(host: SolarSystem) -> Self {
        let mut rng = rand::thread_rng();
        let n = host.get_n_planets() + 1;
        let letter: char = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().nth(n - 1).unwrap();

        let mass = rand_distr::Normal::new(
            1.5,
            0.7,
        ).unwrap().sample(&mut rng.clone()) * consts::EARTH_M_KG;
        
        let density = rand_distr::Normal::new(
            5.5,
            0.75,
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

        Self {
            name,
            mass,
            radius,
            orbit_radius,
            orbit_period,
        }
    }
}

impl CanOrbit for Planet {
    type HostType = SolarSystem;

    fn get_orbit_radius(&self) -> f32 {
        self.orbit_radius
    }

    fn get_orbit_period(&self) -> f32 {
        self.orbit_period
    }
}