pub mod components;
pub mod resources;
mod systems;

use crate::AppState;

use self::resources::*;
use self::systems::*;
use bevy::prelude::*;

use super::GameState;

pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_system(spawn_stars.in_schedule(OnEnter(AppState::InGame)))
            .add_system(despawn_stars.in_schedule(OnExit(AppState::InGame)))
            .add_systems(
                (tick_star_spawn_timer, spawn_stars_over_time)
                    .in_set(OnUpdate(AppState::InGame))
                    .in_set(OnUpdate(GameState::Running)),
            );
    }
}
