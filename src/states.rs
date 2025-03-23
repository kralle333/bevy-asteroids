use crate::asteroid::Asteroid;
use crate::bullet::Bullet;
use crate::collision::{check_bullet_collision, check_player_collision};
use crate::helpers::{despawn_query, despawn_recursive_query};
use crate::lifetime::check_lifetime;
use crate::player::{Player, ShipInvincibility, update_invincibility};
use crate::spawner::Spawner;
use crate::transforming::Physics;
use crate::ui::{LivesUi, MenuButtonText, MenuUi, PlayerLives, Score, ScoreUi, update_scoreboard};
use crate::{player, spawner};
use bevy::DefaultPlugins;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::color::Color;
use bevy::hierarchy::{BuildChildren, ChildBuild, DespawnRecursiveExt};
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_prototype_lyon::draw::Stroke;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::geometry::GeometryBuilder;
use bevy_prototype_lyon::plugin::ShapePlugin;
use bevy_prototype_lyon::shapes;

pub const PLAYER_SIZE: f32 = 16.0;

#[derive(SystemSet, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum GameSets {
    #[default]
    Menu,
    Playing,
}

#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum GameStates {
    #[default]
    Menu,
    Playing,
}

pub fn setup_playing_state(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut player_lives: ResMut<PlayerLives>,
) {
    let player_size = PLAYER_SIZE / 2.0;
    let player_triangle = shapes::Polygon {
        points: vec![
            Vec2::new(-player_size, player_size),
            Vec2::new(-player_size, -player_size),
            Vec2::new(player_size * 2.0, 0.0),
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
        Physics::new(),
        Player,
        ShipInvincibility::new(),
    ));

    player_lives.reset();
    score.clear();
    commands
        .spawn((
            Text::new("Lives: "),
            TextFont {
                font_size: 24.0,
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
                font_size: 24.0,
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
    commands.spawn(Spawner::new());
}

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player::control_player,
                spawner::spawn_objects,
                check_lifetime,
                check_bullet_collision,
                check_player_collision,
                update_invincibility,
                update_scoreboard,
            )
                .in_set(GameSets::Playing),
        );
    }
}

pub fn setup_cam(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (640.0, 480.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_state(GameStates::Menu)
        .add_plugins(ShapePlugin)
        .insert_resource(Score::default())
        .insert_resource(PlayerLives::default())
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup_cam);
    }
}

pub fn setup_menu(mut commands: Commands) {
    commands
        .spawn((
            MenuUi,
            Node {
                // center button
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Percent(20.),
                        height: Val::Percent(6.),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::WHITE),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        MenuButtonText,
                        Text::new("Play"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor::BLACK,
                    ));
                });
        });
}

pub fn cleanup_menu(mut commands: Commands, menu_data: Query<Entity, With<MenuUi>>) {
    for e in &menu_data {
        commands.entity(e).despawn_recursive();
    }
}

pub fn cleanup_playing(
    mut commands: Commands,
    player: Query<Entity, With<Player>>,
    score_ui: Query<Entity, With<ScoreUi>>,
    lives_ui: Query<Entity, With<LivesUi>>,
    spawner: Query<Entity, With<Spawner>>,
    asteroids: Query<Entity, With<Asteroid>>,
    bullets: Query<Entity, With<Bullet>>,
) {
    despawn_recursive_query(&mut commands, score_ui);
    despawn_recursive_query(&mut commands, lives_ui);
    despawn_query(&mut commands, player);
    despawn_query(&mut commands, spawner);
    despawn_query(&mut commands, bullets);
    despawn_query(&mut commands, asteroids);
}
