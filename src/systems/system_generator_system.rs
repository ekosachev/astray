use bevy::prelude::Commands;

use crate::components::star::StarBundle;

pub fn generate_star_system(mut commands: Commands) {
    // Generate a star
    let star = StarBundle::generate();
    commands.spawn(star);
}
