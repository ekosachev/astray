use ordered_float::OrderedFloat;
use rand::distributions::Distribution;
use rand_distr;
use rand_distr::num_traits::ToPrimitive;
use ratatui::style::Color;
use ratatui::widgets::canvas::{Circle, Context};
use serde::{Deserialize, Serialize};

use crate::game::celestial_bodies::{CanOrbit, CelestialBody, CelestialBodyType, Displayable, Orbitable};
use crate::game::celestial_bodies::planet::Planet;
use crate::game::celestial_bodies::star::Star;
use crate::game::helpers::astrophysics;
use crate::game::helpers::consts::AU_M;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolarSystem {
    star: Star,
    planets: Vec<Planet>,
    spacing_factor: OrderedFloat<f32>,
}

impl SolarSystem {
    pub fn get_n_planets(&self) -> usize {
        self.planets.len()
    }

    pub fn get_star_mass(&self) -> f32 { self.star.get_mass() }
    pub fn get_star(&self) -> Star { self.star.clone() }

    pub fn get_inner_limit(&self) -> f32 {
        astrophysics::calculate_system_inner_limit_from_star_radius_and_density(
            self.star.get_radius(),
            astrophysics::calculate_density_from_mass_and_radius(
                self.star.get_mass(),
                self.star.get_radius()
            )
        )
    }

    pub fn get_spacing_factor(&self) -> f32 {
        self.spacing_factor.to_f32().unwrap()
    }

    pub  fn get_nth_orbit_radius(&self, n: u32) -> f32 {
        if !self.planets.is_empty() {
            astrophysics::calculate_nth_orbit(
                self.planets[0].get_orbit_radius(),
                self.get_spacing_factor(),
                n,
            )
        } else {
            0.0
        }
    }

    pub fn has_planets_in_habitable_zone(&self) -> Option<Planet> {
        self.planets.iter().find(
            |p| p.is_inside_habitable_zone()
        ).cloned()
    }
}

impl CelestialBody for SolarSystem {
    type HostType = ();

    fn get_type(&self) -> CelestialBodyType {
        CelestialBodyType::SolarSystem
    }

    fn get_mass(&self) -> f32 {
        let mut r = self.star.get_mass();
        self.planets.iter().for_each(|p| {
            r += p.get_mass()
        });
        r
    }

    fn get_radius(&self) -> f32 {
        self.planets.last().unwrap().get_orbit_radius()
    }

    fn generate(host: ()) -> Self {
        let mut rng = rand::thread_rng();

        let spacing_factor = rand_distr::Normal::new(
            0.4,
            0.2
        ).unwrap().sample(&mut rng);

        let mut system = Self {
            star: Star::generate(()),
            planets: vec![],
            spacing_factor: OrderedFloat(spacing_factor),
        };

        let n_planets: i32 = rand_distr::Normal::new(
            5.0,
            1.0
        ).unwrap().sample(&mut rng) as i32;

        for _ in 0..n_planets {
            system.planets.push(Planet::generate(system.clone()));
        }

        system
    }
}

impl Orbitable for SolarSystem {
    type SatelliteType = Planet;

    fn get_satellites(&self) -> Vec<Self::SatelliteType> {
        self.planets.clone()
    }

    fn update_orbits(&mut self) {
        for planet in self.planets.as_mut_slice() {
            planet.update_orbit_position();
        }
    }
}

impl Displayable for SolarSystem {
    fn get_name(&self) -> String {
        self.star.get_name()
    }

    fn get_properties(&self) -> Vec<Vec<String>> {
        Vec::new()
    }
}

impl SolarSystem {
    pub fn draw_image(
        &self,
        ctx: &mut Context,
    ) {
        ctx.draw(
            &Circle {
                x: 0.0,
                y: 0.0,
                radius: (self.star.get_radius() / AU_M) as f64,
                color: self.star.get_menu_color(),
            }
        );

        self.planets.iter().for_each(
            |p| {
                let radius_au = (p.get_orbit_radius() / AU_M) as f64;

                ctx.draw(
                    &Circle {
                        x: 0.0,
                        y: 0.0,
                        radius: radius_au,
                        color: Color::LightBlue,
                    }
                );
                ctx.draw(
                    &Circle {
                        x: 0.0 + radius_au * p.get_orbit_position().cos() as f64,
                        y: 0.0 + radius_au * p.get_orbit_position().sin() as f64,
                        radius: (p.get_radius() / AU_M) as f64,
                        color: p.get_menu_color(),
                    }
                )
            }
        );
    }
}
