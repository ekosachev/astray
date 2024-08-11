pub mod physics {
    pub mod conversion_ratios {
        pub const SOLAR_MASS_TO_KG: f32 = 1.989e30;
        pub const SOLAR_LUMINOSITY_TO_WHATS: f32 = 3.828e26;
        pub const SOLAR_RADII_TO_M: f32 = 695_700_000.0;
        pub const EARTH_MASS_TO_KG: f32 = 5.972e24;
    }

    pub mod constants {
        pub const G: f32 = 6.67408e-11;
    }
}
