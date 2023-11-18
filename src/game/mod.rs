use bevy::prelude::*;

use self::systems::*;
use crate::{events::*, AppState};
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;

pub mod enemy;
pub mod player;
pub mod score;
pub mod star;
mod systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_event::<GameOver>()
            .add_system(pause_game.in_schedule(OnEnter(AppState::InGame)))
            .add_system(resume_game.in_schedule(OnExit(AppState::InGame)))
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(StarPlugin)
            .add_system(toggle_game_state.run_if(in_state(AppState::InGame)));
    }
}

#[derive(States, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Running,
    Paused,
}
