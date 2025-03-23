use bevy::hierarchy::DespawnRecursiveExt;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Component, Entity, Query, With};
use std::f32::consts::PI;
use std::ops::Range;

pub fn random_range(range: Range<f32>) -> f32 {
    fastrand::f32() * (range.end - range.start) + range.start
}

pub fn despawn_query<T: Component>(commands: &mut Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
pub fn despawn_recursive_query<T: Component>(
    commands: &mut Commands,
    query: Query<Entity, With<T>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn get_random_vel(range: Range<f32>) -> Vec2 {
    Vec2::from_angle(random_range(0.0..PI * 2.0)) * random_range(range)
}
