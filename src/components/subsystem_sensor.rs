use bevy::prelude::*;
use bevy::reflect::Reflect;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct SubsystemSensor {
    range: f32,
    noise_floor: f32,
}
