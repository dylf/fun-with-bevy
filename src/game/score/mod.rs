pub mod resources;
mod systems;

use crate::AppState;

use self::resources::*;
use self::systems::*;
use bevy::prelude::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HighScores>()
            .add_system(insert_score.in_schedule(OnEnter(AppState::InGame)))
            .add_system(remove_score.in_schedule(OnExit(AppState::InGame)))
            .add_system(update_score.run_if(in_state(AppState::InGame)))
            .add_system(update_high_scores)
            .add_system(high_scores_updates);
    }
}
