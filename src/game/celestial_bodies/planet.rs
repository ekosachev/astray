use rand::Rng;
use ratatui::prelude::Color;
use crate::game::celestial_bodies::{CelestialBody, CelestialBodyType};

pub struct Planet {
    name: String,
    mass: f32,
    radius: f32,
}

impl CelestialBody for Planet {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_type(&self) -> CelestialBodyType {
        CelestialBodyType::Planet
    }

    // fn get_menu_color(&self) -> Color {
    //     todo!()
    // }

    fn generate<T: CelestialBody>(host: Option<T>) -> Self {
        let mut rng = rand::thread_rng();
        
        let mass = rng.gen_range(1.0..=10.0) * 1.98 * 10.0f32.powi(24);
        let radius = rng.gen_range(1.0..=10.0) * 1.98 * 10.0f32.powi(6);
        
        Self {
            name: host.unwrap().get_name(),
            mass,
            radius,
        }
    }
}