use ratatui::style;
use crate::game::celestial_bodies::{CelestialBody, CelestialBodyType, constants};
use crate::game::helpers::{ consts, astrophysics };
use rand;
use rand::Rng;
use rand::seq::SliceRandom;
use rand::distributions::Distribution;

#[derive(Clone)]
enum StarClass {
    O,
    B,
    A,
    F,
    G,
    K,
    M,
}

#[derive(Clone)]
pub struct Star {
    name: String,
    class: StarClass,
    mass: f32,
    radius: f32,
    surface_temp: f32,
    
}

impl CelestialBody for Star {
    type HostType = ();
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_type(&self) -> CelestialBodyType {
        CelestialBodyType::Star
    }

    fn get_mass(&self) -> f32 {
        self.mass
    }

    fn get_radius(&self) -> f32 {
        self.radius
    }

    fn get_menu_color(&self) -> style::Color {
        match self.class {
            StarClass::O => { style::Color::Indexed(27) }
            StarClass::B => { style::Color::Indexed(33) }
            StarClass::A => { style::Color::Indexed(195) }
            StarClass::F => { style::Color::Indexed(231) }
            StarClass::G => { style::Color::Indexed(230) }
            StarClass::K => { style::Color::Indexed(216) }
            StarClass::M => { style::Color::Indexed(160) }
        }
    }

    fn generate(host: ()) -> Self {
        let mut rng = rand::thread_rng();


        let mass: f32 = rand_distr::Normal::new(
            1.7,
            0.19
        ).unwrap().sample(&mut rng) * consts::SUN_M_KG;

        let luminosity = astrophysics::calculate_luminosity_from_mass(mass);
        let radius: f32 = astrophysics::calculate_star_radius_from_mass(mass);
        let surface_temp: f32 = astrophysics::calculate_temperature_from_luminosity_and_radius(
            luminosity, radius
        );

        let class = match surface_temp {
            3700.0..=5200.0 => { StarClass::K },
            5200.0..=6000.0 => { StarClass::G },
            6000.0..=7500.0 => { StarClass::F },
            7500.0..=10000.0 => { StarClass::A },
            10000.0..=33000.0 => { StarClass::B },
            33000.0..=95000.0 => { StarClass::O },
            
            _ => { StarClass::M }
        };
        
        let name = constants::STAR_NAMES.choose(&mut rng).unwrap().clone();
        Self {
            name,
            class,
            mass,
            radius,
            surface_temp,
        }
    }
}