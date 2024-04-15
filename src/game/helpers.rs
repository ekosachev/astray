pub mod consts {
    pub const G: f32 = 6.6743e-11;
    pub const AU_M: f32 = 149597870700.0;
}

pub mod orbit_dynamics {
    use crate::game::helpers::consts;
    pub fn calculate_roche_limit(primary_mass: f32, secondary_mass: f32, secondary_radius: f32) -> f32 {
        secondary_mass * f32::cbrt(2f32 * primary_mass / secondary_mass)
    }
    
    pub fn calculate_soi(primary_mass: f32, secondary_mass: f32) -> f32 {
        (secondary_mass / primary_mass).powf(0.4)
    }
    
    pub fn calculate_orbital_velocity(primary_mass: f32, radius: f32) -> f32 {
        (consts::G * primary_mass / radius).sqrt()
    }
}

pub mod astrophysics {
    use std::ops::{Range, RangeInclusive};
    use crate::game::helpers::consts;

    pub fn calculate_luminosity_from_mass(mass: f32) -> f32 {
        // Express mass as a multiple os solar mass
        let mass = mass / 1.9885e30;

        let luminosity_solar = match mass {
            0.0..=0.43 => { 0.23 * mass.powf(2.3)}
            0.44..=2.0 => {mass.powi(2) }
            _ => { 1.4 * mass.powf(3.5) }
        };

        // Return luminosity in watts
        luminosity_solar * 3.828e26
    }

    pub fn calculate_star_radius_from_mass(mass: f32) -> f32 {
        // Express mass as a multiple of solar mass
        let mass = mass / 1.9885e30;

        let radius = match mass {
            0.0..=1.0 => { mass.powf(0.8) },
            _ => { mass.powf(0.57) },
        };

        // Return radius in meters
        radius * 6.957e8
    }

    pub fn calculate_density_from_mass_and_radius(mass: f32, radius: f32) -> f32 {
        mass / radius.powi(3)
    }

    pub fn calculate_inner_radius_of_habitable_zone_from_luminosity(luminosity: f32) -> f32 {
        let r_au = (luminosity / 1.1).sqrt(); // calculate radius in au
        r_au * consts::AU_M // convert to meters
    }

    pub fn calculate_outer_radius_of_habitable_zone_from_luminosity(luminosity: f32) -> f32 {
        let r_au = (luminosity / 0.53).sqrt(); // calculate radius in au
        r_au * consts::AU_M // convert to meters
    }
    
    pub fn calculate_habitable_zone_from_luminosity(luminosity: f32) -> RangeInclusive<f32> {
        let inner = calculate_inner_radius_of_habitable_zone_from_luminosity(luminosity);
        let outer = calculate_outer_radius_of_habitable_zone_from_luminosity(luminosity);
        inner..=outer
    }

    pub fn calculate_frost_line_from_luminosity(luminosity: f32) -> f32 {
        let r_au = 4.85 * luminosity.sqrt(); // calculate radius in au
        r_au * consts::AU_M // convert to meters
    }
    
    pub fn calculate_system_inner_limit_from_star_radius_and_density(radius: f32, density: f32) -> f32 {
        let r_au = 2.455 * (radius * 696340f32) * ((density * 1408f32) / 5400f32).powf(1.0 / 3.0)
            / 149600000f32; // calculate radius in au

        r_au * consts::AU_M // convert to meters
    }
    
    pub fn calculate_n_orbits(first_orbit: f32, spacing: f32, n: usize) -> Vec<f32> {
        let mut result = Vec::<f32>::with_capacity(n);
        result.push(first_orbit);
        
        for i in 0..=(n-2) {
            result.push(first_orbit + spacing * (2i32.pow(i as u32) as f32))
        }
        
        result
    }
}