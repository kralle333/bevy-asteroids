use crate::asteroid::{Asteroid, create_asteroid};
use crate::bullet::Bullet;
use crate::helpers::{get_random_vel, random_range};
use crate::lifetime::Lifetime;
use crate::player::{Player, ShipInvincibility};
use crate::spawner::Spawner;
use crate::states::{GameStates, PLAYER_SIZE};
use crate::transforming::Physics;
use crate::ui::PlayerLives;
use crate::{Score, asteroid};
use bevy::math::{Quat, Vec2, Vec3};
use bevy::prelude::{Commands, Entity, NextState, Query, ResMut, Transform, With, Without};
use bevy::sprite::Sprite;
use bevy_color::Color;

fn spawn_asteroids(commands: &mut Commands, count: i32, pos: Vec2, size: f32) {
    let modifier = asteroid::MAX_SIZE / size;
    for _ in 0..count {
        create_asteroid(
            commands,
            size,
            pos,
            get_random_vel(20.0 * modifier..100.0 * (asteroid::MAX_SIZE / size)),
        );
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
                spawn_explosion(&mut commands, at.translation);
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
    mut players: Query<
        (Entity, &mut Transform, &mut Physics, &mut ShipInvincibility),
        With<Player>,
    >,
    asteroids: Query<(Entity, &Transform, &Asteroid), Without<Player>>,
    mut spawner: Query<&mut Spawner>,
    mut lives: ResMut<PlayerLives>,
    mut next_state: ResMut<NextState<GameStates>>,
) {
    for (pe, mut pt, mut pp, mut si) in &mut players {
        if si.is_invincible() {
            return;
        }
        for (ae, at, aa) in &asteroids {
            let dist_to_ast = pt.translation.distance(at.translation);
            if dist_to_ast < aa.size || dist_to_ast < PLAYER_SIZE / 2.0 {
                commands.entity(ae).despawn();
                let pos = Vec2::new(at.translation.x, at.translation.y);
                spawn_asteroids(&mut commands, 2, pos, aa.size / 2.0);

                if let Ok(mut spawner) = spawner.get_single_mut() {
                    spawner.reset();
                }
                **lives -= 1;
                si.set_invincible();
                pt.translation = Vec3::ZERO;
                pt.rotation = Quat::from_rotation_z(0.0);
                pp.reset();
                if **lives == 0 {
                    commands.entity(pe).despawn();
                    next_state.set(GameStates::Menu);
                }
            }
        }
    }
}

fn spawn_explosion(commands: &mut Commands, position: Vec3) {
    let particles = fastrand::i32(4..10);

    for _ in 0..particles {
        let rand_dir = get_random_vel(30.0..100.0);
        let size = random_range(2.0..3.0);
        commands.spawn((
            Transform::from_translation(position),
            Physics::new_with_vel(rand_dir),
            Lifetime::new(0.8),
            Sprite::from_color(Color::WHITE, Vec2::new(size, size)),
        ));
    }
}
