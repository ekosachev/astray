use rand::Rng;
use ratatui::prelude::Color;
use crate::game::celestial_bodies::{CelestialBody, CelestialBodyType};
use crate::game::celestial_bodies::star::Star;
use crate::game::helpers::orbit_dynamics;

pub struct Planet {
    name: String,
    mass: f32,
    radius: f32,
    orbit_radius: f32,
    orbit_period: f32,
}

impl CelestialBody for Planet {
    type HostType = Star;
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_type(&self) -> CelestialBodyType {
        CelestialBodyType::Planet
    }

    fn get_mass(&self) -> f32 {
        self.mass
    }

    // fn get_menu_color(&self) -> Color {
    //     todo!()
    // }

    fn generate(host: Star) -> Self {
        
        
        let mut rng = rand::thread_rng();
        
        let density = rng.gen_range(3.5..=5.4);
        let mass = rng.gen_range(0.1..=300.0) * 5.97 * 10.0f32.powi(24);
        let volume = mass / density;
        let radius = f32::cbrt((4.0 * volume) / (3.0 * std::f32::consts::PI));
        
        let star_mass = host.get_mass();
        let roche_limit = orbit_dynamics::calculate_roche_limit(
            star_mass, mass, radius
        );
        let soi = orbit_dynamics::calculate_soi(star_mass, mass);
        let orbit_radius = rng.gen_range(roche_limit.sqrt()..=soi.sqrt()).powi(2);
        
        let orbital_velocity = orbit_dynamics::calculate_orbital_velocity(
            star_mass, mass
        );
        let orbit_period = 2.0 * std::f32::consts::PI * orbit_radius / orbital_velocity;
        
        Self {
            name: host.get_name(),
            mass,
            radius,
            orbit_radius,
            orbit_period,
        }
    }
}