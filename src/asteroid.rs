use crate::transforming::Physics;
use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Component, Transform, default};
use bevy_prototype_lyon::draw::Stroke;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::geometry::GeometryBuilder;
use bevy_prototype_lyon::shapes;
use std::f32::consts::PI;
use crate::helpers::random_range;

#[derive(Component)]
pub struct Asteroid {
    pub size: f32,
}


pub const MAX_SIZE: f32 = 50.0;

pub fn create_asteroid(commands: &mut Commands, size: f32, position: Vec2, vel: Vec2) {
    let mut points = vec![];
    let mut angle = 0.0;
    while angle < PI * 2.0 {
        let rand_size = random_range(size / 1.5..size);
        points.push(Vec2::from_angle(angle) * rand_size);
        angle += random_range(PI / 10.0..PI / 6.0);
    }

    let player_triangle = shapes::Polygon {
        points,
        closed: true,
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&player_triangle),
            transform: Transform::from_xyz(position.x, position.y, 0.0),
            ..default()
        },
        Stroke::new(Color::WHITE, 1.0),
        Physics {
            max_vel: Vec2::new(50.0, 50.0),
            rot_vel: random_range(-1.0..1.0),
            vel,
            ..default()
        },
        Asteroid { size },
    ));
}
