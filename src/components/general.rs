use bevy::prelude::Component;

#[derive(Component)]
pub struct Mass(pub f32);

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Position(pub f32, pub f32);

#[derive(Component)]
pub struct Temperature(pub f32);

#[derive(Component)]
pub struct Radius(pub f32);

pub struct Orbit {
    pub radius: f32,
    pub period: f32,
    pub position: f32,
}
