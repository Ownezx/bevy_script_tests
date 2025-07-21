use bevy::prelude::*;
use bevy::reflect::{FromReflect, Reflect};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct SensorTrace {
    biological: f32,
    electrical: f32,
    gravitational: f32,
    size_x: f32,
    size_y: Option<f32>,
}
