use std::f32::consts::TAU;
use bevy::prelude::{Entity, Query, Res, Time};
use crate::components::general::{Orbit, Position, Satellites};

pub fn update_orbits(
    mut data: Query<(Entity, &mut Orbit, &mut Position)>,
    satellite_hosts: Query<(Entity, &Satellites, &Position)>,
    time: Res<Time>,
) {
    for (satellite, mut orbit, mut pos) in data.iter_mut() {
        let delta_theta = TAU / orbit.period * time.delta_seconds() * 100.0;
        orbit.position += delta_theta;
        let host_position = satellite_hosts.iter().find(|(_, ref s, _)| {s.0.contains(&satellite)}).unwrap().2;
        pos.0 = host_position.0 + orbit.radius * orbit.position.cos();
        pos.1 = host_position.1 + orbit.radius * orbit.position.sin();
    }
}