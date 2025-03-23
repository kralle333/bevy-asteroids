use crate::GameStates;
use bevy::hierarchy::Children;
use bevy::prelude::{
    Button, Changed, Component, Deref, DerefMut, Entity, Interaction, NextState, Query, Res,
    ResMut, Resource, Single, Text, TextFont, TextUiWriter, With,
};
#[derive(Component)]
pub struct MenuButtonText;

#[derive(Resource, Deref, DerefMut, Default)]
pub struct Score(usize);

impl Score {
    pub fn clear(&mut self) {
        self.0 = 0
    }
}
#[derive(Resource, Deref, DerefMut, Default)]
pub struct PlayerLives(usize);

impl PlayerLives {
    pub fn reset(&mut self) {
        self.0 = 3
    }
}

#[derive(Component)]
pub struct ScoreUi;

#[derive(Component)]
pub struct LivesUi;

#[derive(Component)]
pub struct MenuUi;

pub fn update_scoreboard(
    lives: Res<PlayerLives>,
    score: Res<Score>,
    lives_root: Single<Entity, (With<LivesUi>, With<Text>)>,
    score_root: Single<Entity, (With<ScoreUi>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*lives_root, 1) = lives.to_string();
    *writer.text(*score_root, 0) = score.to_string();
}

#[allow(clippy::type_complexity)]
pub fn main_menu(
    mut next_state: ResMut<NextState<GameStates>>,
    mut text_query: Query<&mut TextFont>,
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, children) in &mut interaction_query {
        for x in children.iter() {
            if let Ok(mut text) = text_query.get_mut(*x) {
                match *interaction {
                    Interaction::Pressed => {
                        next_state.set(GameStates::Playing);
                        text.font_size = 18.0;
                    }
                    Interaction::Hovered => {
                        text.font_size = 22.0;
                    }
                    Interaction::None => {
                        text.font_size = 18.0;
                    }
                }
            }
        }
    }
}
