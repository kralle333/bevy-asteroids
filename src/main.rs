mod asteroid;
mod bullet;
mod collision;
mod player;
mod transforming;
mod helpers;

use crate::bullet::check_bullet_lifetime;
use crate::collision::{check_bullet_collision, check_player_collision};
use crate::player::update_invincibility;
use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;
use bevy_prototype_lyon::prelude::{GeometryBuilder, ShapeBundle, Stroke};
use bevy_prototype_lyon::shapes;
use player::Player;
use std::f32::consts::PI;
use transforming::Physics;
use crate::helpers::random_range;

#[derive(Component)]
struct Spawner {
    next_spawn: f32,
}

#[derive(Resource, Deref, DerefMut)]
struct Score(usize);

#[derive(Component)]
struct ScoreUi;

#[derive(Component)]
struct LivesUi;

fn update_scoreboard(
    score: Res<Score>,
    lives_root: Single<Entity, (With<LivesUi>, With<Text>)>,
    score_root: Single<Entity, (With<ScoreUi>, With<Text>)>,
    player: Single<&Player>,
    mut writer: TextUiWriter,
) {
    *writer.text(*lives_root, 1) = player.lives.to_string();
    *writer.text(*score_root, 0) = score.to_string();
}

fn spawn_objects(
    mut commands: Commands,
    window: Query<&Window>,
    time: Res<Time>,
    mut query: Query<&mut Spawner>,
) {
    let mut spawner = query.single_mut();
    spawner.next_spawn -= time.delta_secs();
    if spawner.next_spawn <= 0.0 {
        spawner.next_spawn = 5.0;

        let window = window.single();
        let dims = Vec2::new(window.width(), window.height());

        let angle = random_range(0.0..PI * 2.0);
        let dir = Vec2::from_angle(angle);

        let pos = (dir * dims) - dims / 2.0;
        let vel = Vec2::from_angle(random_range(0.0..PI * 2.0)) * random_range(1.0..10.0);

        let size = random_range(40.0..70.0);
        asteroid::create_asteroid(&mut commands, size, pos, vel);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    let player_triangle = shapes::Polygon {
        points: vec![
            Vec2::new(-5.0, 5.0),
            Vec2::new(-5.0, -5.0),
            Vec2::new(10.0, 0.0),
        ],
        closed: true,
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&player_triangle),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Stroke::new(Color::WHITE, 1.0),
        Physics {
            max_vel: Vec2::new(200.0, 200.0),
            ..default()
        },
        Player::new(3),
    ));

    commands
        .spawn((
            Text::new("Lives: "),
            TextFont {
                font_size: 32.0,
                ..default()
            },
            TextColor(Color::WHITE),
            LivesUi,
            Node {
                position_type: PositionType::Absolute,
                ..default()
            },
        ))
        .with_child((
            TextSpan::default(),
            TextFont {
                font_size: 32.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
    commands.spawn((
        Text::new(""),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::WHITE),
        ScoreUi,
        Node {
            position_type: PositionType::Relative,
            flex_direction: FlexDirection::Column,
            justify_self: JustifySelf::Center,
            ..default()
        },
    ));
    commands.spawn(Spawner { next_spawn: 0.0 });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin) // Add this!
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .insert_resource(Score(0))
        .add_systems(Startup, setup)
        .add_systems(Update, player::control_player)
        .add_systems(Update, transforming::move_objects)
        .add_systems(Update, transforming::wrap_objects)
        .add_systems(Update, spawn_objects)
        .add_systems(Update, check_bullet_lifetime)
        .add_systems(Update, check_bullet_collision)
        .add_systems(Update, check_player_collision)
        .add_systems(Update, update_invincibility)
        .add_systems(Update, update_scoreboard)
        .run();
}
