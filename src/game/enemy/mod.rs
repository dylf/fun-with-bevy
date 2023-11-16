use bevy::prelude::*;

use crate::AppState;

use self::{resources::*, systems::*};

use super::GameState;

pub mod components;
pub mod resources;
mod systems;

pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_system(spawn_enemies.in_schedule(OnEnter(AppState::InGame)))
            .add_system(despawn_enemies.in_schedule(OnExit(AppState::InGame)))
            .add_systems(
                (
                    update_enemy_direction,
                    enemy_movement,
                    confine_enemy_movement,
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time,
                )
                    .in_set(OnUpdate(AppState::InGame))
                    .in_set(OnUpdate(GameState::Running)),
            );
    }
}
