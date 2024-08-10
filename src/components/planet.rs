use std::f32::consts::PI;
use bevy::ecs::observer::TriggerTargets;
use bevy::prelude::Entity;
use rand::distributions::Distribution;
use crate::components::general::{Mass, Name, Orbit, Radius};
use crate::components::star::StarBundle;

pub struct PlanetBundle {
    name: Name,
    mass: Mass,
    radius: Radius,
    orbit: Orbit,
}

impl PlanetBundle {
    pub fn generate(star: &StarBundle, n: usize) -> Self {
        let mut rng = rand::thread_rng();
        let letter: char = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().nth(n).unwrap();
        
        let mass = rand_distr::Normal::new(
            1.5,
            0.7,
        ).unwrap().sample(&mut rng) * EARTH_MASS_TO_KG;
        
        let density = rand_distr::Normal::new(
            5500.0,
            750.0,
        ).unwrap().sample(&mut rng);
        
        let volume = mass / density;
        let radius = (volume / ((4.0 / 3.0) * PI).cbrt();
        
        let orbit_radius = if n == 1 {
            let roche_limit = star.roche_limit();
            roche_limit + rand_distr::Normal::new(
                0.4,
                0.2,
            ).unwrap().sample(&mut rng) * AU_TO_M;
        } else {
            
        }
        
        Self {
            name,
            mass,
            radius,
            orbit: Orbit {
                radius: 0.0,
                period: 0.0,
                position: 0.0,
            }
        }
    }
}
