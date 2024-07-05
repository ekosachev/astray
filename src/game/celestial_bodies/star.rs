use ordered_float::OrderedFloat;
use rand;
use rand::distributions::Distribution;
use rand::seq::SliceRandom;
use rand_distr::num_traits::ToPrimitive;
use ratatui::style;
use serde::{Deserialize, Serialize};

use crate::game::celestial_bodies::{CelestialBody, CelestialBodyType, constants, Displayable};
use crate::game::helpers::{astrophysics, consts};
use crate::game::helpers::astrophysics::{
    calculate_inner_radius_of_habitable_zone_from_luminosity,
    calculate_luminosity_from_mass,
    calculate_outer_radius_of_habitable_zone_from_luminosity,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
enum StarClass {
    O,
    B,
    A,
    F,
    G,
    K,
    M,
}

impl From<StarClass> for char {
    fn from(value: StarClass) -> Self {
        match value {
            StarClass::O => { 'O' }
            StarClass::B => { 'B' }
            StarClass::A => { 'A' }
            StarClass::F => { 'F' }
            StarClass::G => { 'G' }
            StarClass::K => { 'K' }
            StarClass::M => { 'M' }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Star {
    name: String,
    class: StarClass,
    mass: OrderedFloat<f32>,
    radius: OrderedFloat<f32>,
    surface_temp: OrderedFloat<f32>,
    
}

impl Star {
    pub fn get_luminosity(&self) -> f32 {
        calculate_luminosity_from_mass(self.get_mass())
    }
}

impl CelestialBody for Star {
    type HostType = ();

    fn get_type(&self) -> CelestialBodyType {
        CelestialBodyType::Star
    }

    fn get_mass(&self) -> f32 {
        self.mass.to_f32().unwrap()
    }

    fn get_radius(&self) -> f32 {
        self.radius.to_f32().unwrap()
    }

    fn generate(host: ()) -> Self {
        let mut rng = rand::thread_rng();


        let mass_solar: f32 = rand_distr::Normal::new(
            1.2,
            0.2,
        ).unwrap().sample(&mut rng);

        let mass = mass_solar
            .min(10.0)
            .max(0.1) * consts::SUN_M_KG;

        let luminosity = astrophysics::calculate_luminosity_from_mass(mass);
        let radius: f32 = astrophysics::calculate_star_radius_from_mass(mass);
        let surface_temp: f32 = astrophysics::calculate_temperature_from_luminosity_and_radius(
            luminosity, radius
        );

        let class = match surface_temp as i32 {
            3700..=5200 => { StarClass::K },
            5201..=6000 => { StarClass::G },
            6001..=7500 => { StarClass::F },
            7501..=10000 => { StarClass::A },
            10001..=33000 => { StarClass::B },
            33001..=95000 => { StarClass::O },
            
            _ => { StarClass::M }
        };
        
        let name = constants::STAR_NAMES.choose(&mut rng).unwrap().clone();
        Self {
            name,
            class,
            mass: OrderedFloat(mass),
            radius: OrderedFloat(radius),
            surface_temp: OrderedFloat(surface_temp),
        }
    }
}

impl Displayable for Star {
    fn get_properties(&self) -> Vec<Vec<String>> {
        let hz_inner = calculate_inner_radius_of_habitable_zone_from_luminosity(
            self.get_luminosity()
        );

        let hz_outer = calculate_outer_radius_of_habitable_zone_from_luminosity(
            self.get_luminosity()
        );


        vec![
            vec![
                String::from("Mass"),
                format!("{:.3E} kg", self.mass),
                format!("{:.3} solar masses", self.mass / consts::SUN_M_KG),
            ],
            vec![
                String::from("Radius"),
                format!("{:.3E} m", self.radius),
                format!("{:.3} solar radii", self.radius / consts::SUN_R_M),
            ],
            vec![
                String::from("Luminosity"),
                format!("{:.3E} W", astrophysics::calculate_luminosity_from_mass(self.get_mass())),
                format!("{:.3} solar luminosities", astrophysics::calculate_luminosity_from_mass
                    (self.get_mass()) / consts::SUN_LUM_W),
            ],
            vec![
                String::from("Temperature"),
                format!("{:.3E} K", self.surface_temp),
                format!("{:.3} solar temperatures", self.surface_temp / consts::SUN_T_K),
            ],
            vec![
                String::from("Inner radius of habitable zone"),
                format!("{:.3E} m", hz_inner),
                format!("{:.3} AU", hz_inner / consts::AU_M),
            ],
            vec![
                String::from("Outer radius of habitable zone"),
                format!("{:.3E} m", hz_outer),
                format!("{:.3} AU", hz_outer / consts::AU_M),
            ],
        ]
    }

    fn get_name(&self) -> String {
        self.name.clone()
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
}