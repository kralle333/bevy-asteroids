use crate::asteroid::{Asteroid, create_asteroid};
use crate::bullet::Bullet;
use crate::player::Player;
use crate::transforming::Physics;
use crate::{Score, asteroid};
use bevy::math::{Quat, Vec2, Vec3};
use bevy::prelude::{Commands, Entity, Query,  ResMut, Single, Text, Transform, With, Without};
use std::f32::consts::PI;
use crate::helpers::random_range;

fn spawn_asteroids(mut commands: &mut Commands, count: i32, pos: Vec2, size: f32) {
    let get_vel = || {
        Vec2::from_angle(random_range(0.0..PI * 2.0))
            * random_range(10.0..50.0 * (asteroid::MAX_SIZE / size))
    };
    for _ in 0..count {
        create_asteroid(&mut commands, size, pos, get_vel());
    }
}

pub fn check_bullet_collision(
    mut score: ResMut<Score>,
    mut commands: Commands,
    bullets: Query<(Entity, &Transform), With<Bullet>>,
    asteroids: Query<(Entity, &Transform, &Asteroid)>,
) {
    for (be, bt) in &bullets {
        for (ae, at, aa) in &asteroids {
            let dist = bt.translation.distance(at.translation);
            if dist < 4.0 || dist < aa.size {
                **score += 1;
                commands.entity(be).despawn();
                commands.entity(ae).despawn();
                if aa.size < 20.0 {
                    continue;
                }
                let pos = Vec2::new(at.translation.x, at.translation.y);
                spawn_asteroids(&mut commands, 2, pos, aa.size / 2.0);
            }
        }
    }
}
pub fn check_player_collision(
    mut commands: Commands,
    player: Single<(Entity, &mut Transform, &mut Physics, &mut Player)>,
    asteroids: Query<(Entity, &Transform, &Asteroid), Without<Player>>,
) {
    let (pe, mut pt, mut pp, mut player) = player.into_inner();
    if player.invincibility_time > 0.0 {
        return;
    }
    for (ae, at, aa) in &asteroids {
        if pt.translation.distance(at.translation) < aa.size {
            commands.entity(ae).despawn();
            let pos = Vec2::new(at.translation.x, at.translation.y);
            spawn_asteroids(&mut commands, 2, pos, aa.size / 2.0);

            player.lives -= 1;
            player.set_invincible();
            pt.translation = Vec3::ZERO;
            pt.rotation = Quat::from_rotation_z(0.0);
            pp.reset();
            if player.lives == 0 {
                commands.entity(pe).despawn();
                commands.spawn((Text::new("Game Over"), Transform::default()));
            }
        }
    }
}
