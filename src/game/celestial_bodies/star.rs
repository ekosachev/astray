use ratatui::style;
use crate::game::celestial_bodies::{CelestialBody, CelestialBodyType, constants};
use rand;
use rand::Rng;
use rand::seq::SliceRandom;

enum StarClass {
    O,
    B,
    A,
    F,
    G,
    K,
    M,
}

pub struct Star {
    name: String,
    class: StarClass,
    mass: f32,
    radius: f32,
    surface_temp: i32,
    
}

impl CelestialBody for Star {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_type(&self) -> CelestialBodyType {
        CelestialBodyType::Star
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

    fn generate<T: CelestialBody>(host: Option<T>) -> Self {
        let mut rng = rand::thread_rng();

        let class: StarClass = match rng.gen_range(0..=100i32) {
            0..=1    => StarClass::O,
            2..=3    => StarClass::B,
            4..=5    => StarClass::A,
            6..=7    => StarClass::F,
            8..=14   => StarClass::G,
            15..=26  => StarClass::K,
            27..=100 => StarClass::M,
            _ => unreachable!()
        };

        let mass: f32 = rng.gen_range(
            match class {
                StarClass::O => { 16.0..=150.0 }
                StarClass::B => { 2.10..=16.00 }
                StarClass::A => { 1.40..=2.100 }
                StarClass::F => { 1.04..=1.400 }
                StarClass::G => { 0.80..=1.040 }
                StarClass::K => { 0.45..=0.800 }
                StarClass::M => { 0.08..=0.450 }
            }
        ) * 1.98 * 10.0f32.powi(18);

        let radius: f32 = rng.gen_range(
            match class {
                StarClass::O => { 6.60..=100.0 }
                StarClass::B => { 1.80..=6.600 }
                StarClass::A => { 1.40..=1.800 }
                StarClass::F => { 1.15..=1.400 }
                StarClass::G => { 0.96..=1.150 }
                StarClass::K => { 0.70..=0.960 }
                StarClass::M => { 0.10..=0.700 }
            }
        ) * 6.957 * 10.0f32.powi(5);
        
        let surface_temp: i32 = rng.gen_range(
            match class {
                StarClass::O => { 33_000..=50_000 }
                StarClass::B => { 10_000..=33_000 }
                StarClass::A => { 7_300..=10_000 }
                StarClass::F => { 6_000..=7_300 }
                StarClass::G => { 5_300..=6_000 }
                StarClass::K => { 3_900..=5_300 }
                StarClass::M => { 2_300..=3_900 }
            }
        );
        
        Self {
            name: constants::STAR_NAMES.choose(&mut rng).unwrap().clone(),
            class,
            mass,
            radius,
            surface_temp,
        }
    }
}