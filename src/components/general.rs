use bevy::prelude::{Component, Entity};
use ratatui::prelude::Color;

#[derive(Component, Clone)]
pub struct Mass(pub f32);

#[derive(Component, Clone)]
pub struct Name(pub String);

#[derive(Component, Clone)]
pub struct Position(pub f32, pub f32);

#[derive(Component)]
pub struct Temperature(pub f32);

#[derive(Component, Clone)]
pub struct Radius(pub f32);

#[derive(Component, Clone)]
pub struct Orbit {
    pub radius: f32,
    pub period: f32,
    pub position: f32,
}

#[derive(Component, Clone)]
pub struct Satellites(pub Vec<Entity>);

#[derive(Component, Clone)]
pub struct Renderable(pub Color);

#[derive(Component, Clone, Copy)]
pub struct SolarSystem {}

#[derive(Component, Clone)]
pub struct BelongsToSolarSystem(pub Entity);