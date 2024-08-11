use bevy::prelude::{Commands, ResMut};
use rand::distributions::Distribution;

use crate::components::planet::PlanetBundle;
use crate::components::star::StarBundle;
use crate::CurrentSystem;

pub fn generate_star_system(mut commands: Commands, mut cs: ResMut<CurrentSystem>) {
    // Generate a star
    let mut star = StarBundle::generate();

    let num_planets = rand_distr::Normal::new(5.0, 1.0)
        .unwrap()
        .sample(&mut rand::thread_rng()) as usize;

    let mut planets: Vec<PlanetBundle> = Vec::with_capacity(num_planets);

    for _ in 0..num_planets {
        let planet = PlanetBundle::generate(&star, planets.clone());
        star.satellites.0.push(commands.spawn(planet.clone()).id());
        planets.push(planet);
    }

    cs.0 = Some(commands.spawn(star).id());
}
