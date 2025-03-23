mod asteroid;
mod bullet;
mod collision;
mod helpers;
mod lifetime;
mod player;
mod spawner;
mod states;
mod transforming;
mod ui;

use crate::states::{
    GameSets, GameStates, InitPlugin, PlayingPlugin, cleanup_menu, cleanup_playing, setup_menu,
    setup_playing_state,
};
use crate::ui::{Score, main_menu};
use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins((InitPlugin, PlayingPlugin))
        // Run in menu state and playing state
        .add_systems(Update, transforming::move_objects)
        .add_systems(Update, transforming::wrap_objects)
        .add_systems(Update, main_menu.in_set(GameSets::Menu))
        .add_systems(
            OnEnter(GameStates::Playing),
            (cleanup_menu, cleanup_playing, setup_playing_state).chain(),
        )
        .add_systems(OnEnter(GameStates::Menu), setup_menu);

    app.configure_sets(
        Update,
        GameSets::Playing.run_if(in_state(GameStates::Playing)),
    );
    app.configure_sets(Update, GameSets::Menu.run_if(in_state(GameStates::Menu)));
    app.run();
}
