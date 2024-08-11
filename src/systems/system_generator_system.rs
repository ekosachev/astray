use bevy::prelude::{BuildChildren, Commands};
use rand::Rng;

use crate::components::planet::PlanetBundle;
use crate::components::star::StarBundle;

pub fn generate_star_system(mut commands: Commands) {
    // Generate a star
    let star = StarBundle::generate();

    let num_planets = rand_distr::Normal::new(5.0, 1.0)
        .unwrap()
        .sample(&mut rand::thread_rng()) as usize;

    let mut planets: Vec<PlanetBundle> = Vec::with_capacity(num_planets);

    for _ in 0..num_planets {
        planets.push(PlanetBundle::generate(&star, planets.clone()))
    }

    commands.spawn(star).with_children(|cb| {
        planets.iter().for_each(|p| {
            cb.spawn(p);
        })
    });
}
