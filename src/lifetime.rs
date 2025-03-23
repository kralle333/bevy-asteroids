use bevy::prelude::{Commands, Component, Entity, Query, Res, Time};

#[derive(Component)]
pub struct Lifetime {
    time_left: f32,
}

impl Lifetime {
    pub fn new(time_left: f32) -> Self {
        Self { time_left }
    }
}

pub fn check_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut lifetimes: Query<(Entity, &mut Lifetime)>,
) {
    for (entity, mut lifetime) in &mut lifetimes {
        lifetime.time_left -= time.delta_secs();
        if lifetime.time_left <= 0.0 {
            commands.entity(entity).despawn()
        }
    }
}
