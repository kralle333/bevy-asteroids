use bevy::math::{Quat, Vec2, Vec3};
use bevy::prelude::{Component, Query, Res, Time, Transform, Window, With};
use std::ops::Add;

#[derive(Component, Default)]
pub struct Physics {
    pub acc: Vec2,
    pub vel: Vec2,
    pub max_vel: Vec2,
    pub rot_vel: f32,
}

impl Physics {
    pub fn new() -> Physics {
        Self {
            acc: Vec2::ZERO,
            vel: Vec2::ZERO,
            max_vel: Vec2::ONE * 200.0,
            rot_vel: 0.0,
        }
    }
    pub fn new_with_vel(vel: Vec2) -> Physics {
        Self {
            acc: Vec2::ZERO,
            vel,
            max_vel: Vec2::ONE * 200.0,
            rot_vel: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.acc = Vec2::ZERO;
        self.vel = Vec2::ZERO;
        self.rot_vel = 0.0;
    }
}

pub fn move_objects(time: Res<Time>, mut query: Query<(&mut Transform, &mut Physics)>) {
    for (mut transform, mut physics) in &mut query {
        transform.translation += Vec3::new(physics.vel.x, physics.vel.y, 0.0) * time.delta_secs();

        physics.vel = physics.vel.add(physics.acc * time.delta_secs());
        physics.vel = physics.vel.clamp(-physics.max_vel, physics.max_vel);

        transform.rotation *= Quat::from_rotation_z(physics.rot_vel * time.delta_secs());
    }
}

pub fn wrap_objects(window: Query<&Window>, mut query: Query<&mut Transform, With<Physics>>) {
    let window = window.single();

    for mut transform in &mut query {
        if transform.translation.y > window.height() / 2.0 {
            transform.translation.y -= window.height();
        }
        if transform.translation.y < -window.height() / 2.0 {
            transform.translation.y += window.height();
        }
        if transform.translation.x > window.width() / 2.0 {
            transform.translation.x -= window.width();
        }
        if transform.translation.x < -window.width() / 2.0 {
            transform.translation.x += window.width();
        }
    }
}
