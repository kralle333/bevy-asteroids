use crate::bullet::Bullet;
use crate::transforming::Physics;
use bevy::color::Color;
use bevy::input::ButtonInput;
use bevy::math::{EulerRot, Vec2, Vec3};
use bevy::prelude::{Commands, Component, KeyCode, Query, Res, Single, Sprite, Time, Transform, Visibility, With, default};
use std::f32::consts::PI;
use std::ops::{Add};

#[derive(Component)]
pub struct Player {
    pub(crate) lives: u32,
    pub(crate) invincibility_time: f32,
    invincibility_timer: f32,
}

const FLASHING_SPEED:f32 = 0.2;

impl Player {
    pub fn new(lives: u32) -> Self {
        let mut s = Self {
            lives,
            invincibility_time: 0.0,
            invincibility_timer: 0.0,
        };
        s.set_invincible();
        s
    }
    pub fn set_invincible(&mut self) {
        self.invincibility_time = 2.0;
    }
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
        if keys.pressed(KeyCode::Space) {
            physics.vel = physics.vel.add(dir * 2.0);
        }

        if keys.just_pressed(KeyCode::ControlLeft) {
            let bullet_dir = dir * 200.0;
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
                Bullet { time_left: 4.0 },
            ));
        }
    }
}

pub fn update_invincibility(time: Res<Time>, player: Single<(&mut Visibility, &mut Player)>) {
    let (mut v, mut p) = player.into_inner();

    if p.invincibility_time > 0.0 {
        p.invincibility_time -= time.delta_secs();
    }else {
        return;
    }

    if p.invincibility_time < 0.0 {
        p.invincibility_time = 0.0;
        p.invincibility_timer = 0.0;
        *v = Visibility::Visible;
        return;
    }
    p.invincibility_timer += time.delta_secs();
    if p.invincibility_timer > FLASHING_SPEED{
        p.invincibility_timer = 0.0;
        let next = match v.as_ref() {
            Visibility::Visible => Visibility::Hidden,
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Visible
        };
        *v = next;
    }
}
