use bevy::prelude::{Commands, Component, Entity, Query, Res};
use bevy::time::Time;

#[derive(Component)]
pub struct Bullet {
    pub(crate) time_left: f32,
}

pub fn check_bullet_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut bullets: Query<(Entity, &mut Bullet)>,
) {
    for (entity, mut bullet) in &mut bullets {
        bullet.time_left -= time.delta_secs();
        if bullet.time_left <= 0.0 {
            commands.entity(entity).despawn()
        }
    }
}
