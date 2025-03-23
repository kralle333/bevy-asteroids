use crate::asteroid;
use crate::helpers::random_range;
use crate::player::Player;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Commands, Component, Query, Res, Time, Transform, Window, With};
use std::f32::consts::PI;

#[derive(Component)]
pub struct Spawner {
    next_spawn: f32,
    next_spawn_time: f32,
    spawned_count: i32,
}

impl Spawner {
    pub fn new() -> Self {
        let mut s = Self {
            next_spawn: 0.0,
            next_spawn_time: 0.0,
            spawned_count: 0,
        };
        s.reset();
        s
    }

    pub fn reset(&mut self) {
        self.next_spawn_time = 6.0;
        self.next_spawn = 0.0;
    }
}

pub fn spawn_objects(
    mut commands: Commands,
    window: Query<&Window>,
    player: Query<&Transform, With<Player>>,
    time: Res<Time>,
    mut query: Query<&mut Spawner>,
) {
    let Ok(pt) = player.get_single() else {
        return;
    };
    let mut spawner = query.single_mut();
    spawner.next_spawn -= time.delta_secs();
    if spawner.next_spawn > 0.0 {
        return;
    }

    spawner.spawned_count += 1;
    if spawner.spawned_count % 10 == 0 {
        spawner.next_spawn_time *= 0.9;
    }

    spawner.next_spawn = spawner.next_spawn_time;

    let window = window.single();
    loop {
        let dims = Vec2::new(window.width(), window.height());

        let angle = random_range(0.0..PI * 2.0);
        let dir = Vec2::from_angle(angle);

        let pos = (dir * dims) - dims / 2.0;
        let vel = Vec2::from_angle(random_range(0.0..PI * 2.0)) * random_range(10.0..20.0);

        let size = random_range(40.0..120.0);
        if pt.translation.distance(Vec3::new(pos.x, pos.y, 0.0)) > size * 1.5 {
            asteroid::create_asteroid(&mut commands, size, pos, vel);
            return;
        }
    }
}
