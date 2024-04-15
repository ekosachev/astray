use crate::game::celestial_bodies::{CelestialBody, CelestialBodyType};
use crate::game::celestial_bodies::planet::Planet;
use crate::game::celestial_bodies::star::Star;

pub struct SolarSystem {
    star: Star,
    planets: Vec<Planet>,
}

impl CelestialBody for SolarSystem {
    fn get_name(&self) -> String {
        self.star.get_name()
    }

    fn get_type(&self) -> CelestialBodyType {
        todo!()
    }

    fn get_mass(&self) -> f32 {
        todo!()
    }

    fn generate<T: CelestialBody>(host: Option<T>) -> Self {
        todo!()
    }
}