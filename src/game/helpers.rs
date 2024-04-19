pub mod consts {

    // --- PHYSICAL CONSTANTS ---
    pub const G: f32 = 6.6743e-11;

    // --- CONVERSION RATIOS ---
    pub const AU_M: f32 = 149_597_870_691.0;
    pub const S_YR: i32 = 365 * 24 * 60 * 60;

    // --- SUN-RELATIVE UNITS ---
    pub const SUN_M_KG: f32 = 1.989e30;
    pub const SUN_R_M: f32 = 695_700_000.0;
    pub const SUN_T_K: f32 = 5_800.0;
    pub const SUN_LUM_W: f32 = 3.828e26;

    // --- EARTH-RELATIVE UNITS ---
    pub const EARTH_M_KG: f32 = 5.972e24;
    pub const EARTH_R_M: f32 = 6_378_000.0;
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

pub mod geometry {
    pub fn calculate_sphere_volume_from_radius(radius: f32) -> f32 {
        4.0 / 3.0 * std::f32::consts::PI * radius.powf(3.0)
    }
    
    pub fn calculate_sphere_radius_from_volume(volume: f32) -> f32 {
        (volume / (4.0 / 3.0 * std::f32::consts::PI)).powf(1.0 / 3.0)
    }
}

pub mod astrophysics {
    use std::ops::{Range, RangeInclusive};
    use crate::game::helpers::consts;

    pub fn calculate_luminosity_from_mass(mass: f32) -> f32 {
        // Express mass as a multiple os solar mass
        let mass = mass / consts::SUN_M_KG;

        let luminosity_solar = match mass {
            0.0..=0.43 => { 0.23 * mass.powf(2.3)}
            0.44..=2.0 => { mass.powi(4) }
            _ => { 1.4 * mass.powf(3.5) }
        };

        // Return luminosity in watts
        luminosity_solar * consts::SUN_LUM_W
    }

    pub fn calculate_star_radius_from_mass(mass: f32) -> f32 {
        // Express mass as a multiple of solar mass
        let mass_solar = mass / consts::SUN_M_KG;

        let radius = match mass_solar {
            0.0..=1.0 => { mass_solar.powf(0.8) },
            _ => { mass_solar.powf(0.57) },
        };

        // Return radius in meters
        radius * consts::SUN_R_M
    }

    pub fn calculate_density_from_mass_and_radius(mass: f32, radius: f32) -> f32 {
        mass / radius.powi(3)
    }

    pub fn calculate_temperature_from_luminosity_and_radius(luminosity: f32, radius: f32) -> f32 {
        let lum_solar = luminosity / consts::SUN_LUM_W;
        let r_solar = radius / consts::SUN_R_M;
        ((lum_solar / r_solar.powi(2)).powf(0.25)) * 5776.0
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
        2.455 * radius * (density / 5400f32).powf(1.0 / 3.0)
    }

    pub fn calculate_nth_orbit(first_orbit: f32, spacing: f32, n: u32) -> f32 {
        first_orbit + spacing * (2i32.pow(n) as f32) * consts::AU_M
    }

    pub fn calculate_n_orbits(first_orbit: f32, spacing: f32, n: usize) -> Vec<f32> {
        let mut result = Vec::<f32>::with_capacity(n);
        result.push(first_orbit);
        
        for i in 0u32..=((n-2) as u32) {
            result.push(calculate_nth_orbit(first_orbit, spacing, i))
        }
        
        result
    }
}