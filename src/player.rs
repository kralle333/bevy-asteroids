use crate::bullet::Bullet;
use crate::helpers::random_range;
use crate::lifetime::Lifetime;
use crate::states::PLAYER_SIZE;
use crate::transforming::Physics;
use bevy::color::Color;
use bevy::input::ButtonInput;
use bevy::math::{EulerRot, Vec2, Vec3};
use bevy::prelude::{
    Commands, Component, KeyCode, Query, Res, Sprite, Time, Transform, Visibility, With, default,
};
use std::f32::consts::PI;
use std::ops::Add;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct ShipInvincibility {
    invincibility_time: f32,
    flash_timer: f32,
}

impl ShipInvincibility {
    pub fn new() -> Self {
        let mut s = ShipInvincibility {
            invincibility_time: 0.0,
            flash_timer: 0.0,
        };
        s.set_invincible();
        s
    }
    pub fn is_invincible(&self) -> bool {
        self.invincibility_time > 0.0
    }
    pub fn set_invincible(&mut self) {
        self.invincibility_time = 2.0;
        self.flash_timer = 0.0;
    }
}

const FLASHING_SPEED: f32 = 0.2;

fn random_orthogonal_position(origin: Vec2, direction: Vec2, range: f32) -> Vec2 {
    // Pick left (-1) or right (+1)
    let side = if fastrand::bool() { 1.0 } else { -1.0 };
    // Get orthogonal direction and scale it randomly within the range
    let ortho = Vec2::new(-direction.y, direction.x) * side;
    origin + ortho * random_range(0.0..range)
}
pub fn control_player(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Physics), With<Player>>,
) {
    for (mut transform, mut physics) in &mut query {
        let rotation_speed = 2.0 * PI * time.delta_secs(); // Rotate 2 PI radians per second
        if keys.pressed(KeyCode::ArrowLeft) {
            transform.rotate_z(rotation_speed);
        }
        if keys.pressed(KeyCode::ArrowRight) {
            transform.rotate_z(-rotation_speed);
        }

        let angle = transform.rotation.to_euler(EulerRot::ZXY).0;
        let dir = Vec2::from_angle(angle);
        if keys.pressed(KeyCode::KeyZ) {
            physics.vel = physics.vel.add(dir * 2.0);

            if fastrand::bool() {
                let bottom_pos = transform.translation.truncate() + (-dir * PLAYER_SIZE / 2.0);
                let rand_pos = random_orthogonal_position(bottom_pos, dir, PLAYER_SIZE / 3.0);

                let particle_dir = Vec2::new(-dir.x, -dir.y);
                let speed = random_range(80.0..160.0);
                let size = random_range(0.4..1.0);
                commands.spawn((
                    Transform::from_xyz(rand_pos.x, rand_pos.y, 0.0),
                    Sprite::from_color(Color::WHITE, Vec2::new(size * 2.0, size)),
                    Lifetime::new(0.2),
                    Physics::new_with_vel(particle_dir * speed),
                ));
            }
        }

        if keys.just_pressed(KeyCode::KeyX) {
            let bullet_dir = dir * 250.0;
            commands.spawn((
                Sprite::from_color(Color::WHITE, Vec2::new(4.0, 4.0)),
                Transform::from_translation(
                    transform.translation + (Vec3::new(dir.x, dir.y, 0.0) * 5.0),
                ),
                Physics {
                    vel: bullet_dir,
                    max_vel: Vec2::ONE * 200.0,
                    ..default()
                },
                Bullet,
                Lifetime::new(2.0),
            ));
        }
    }
}

pub fn update_invincibility(
    time: Res<Time>,
    mut player: Query<(&mut Visibility, &mut ShipInvincibility), With<Player>>,
) {
    for (mut v, mut si) in &mut player {
        if si.is_invincible() {
            si.invincibility_time -= time.delta_secs();
        } else {
            return;
        }

        if si.invincibility_time < 0.0 {
            si.invincibility_time = 0.0;
            si.flash_timer = 0.0;
            *v = Visibility::Visible;
            return;
        }
        si.flash_timer += time.delta_secs();
        if si.flash_timer > FLASHING_SPEED {
            si.flash_timer = 0.0;
            let next = match v.as_ref() {
                Visibility::Visible => Visibility::Hidden,
                Visibility::Hidden => Visibility::Visible,
                _ => Visibility::Visible,
            };
            *v = next;
        }
    }
}
