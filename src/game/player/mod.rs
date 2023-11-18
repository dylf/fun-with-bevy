pub mod components;
mod systems;

use crate::AppState;

use self::systems::*;
use bevy::prelude::*;

use super::GameState;

pub const PLAYER_SIZE: f32 = 64.0; // The player sprite size
pub const PLAYER_SPEED: f32 = 500.0;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet {
    Movement,
    Confinement,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(PlayerSystemSet::Movement.before(PlayerSystemSet::Confinement))
            .add_system(spawn_player.in_schedule(OnEnter(AppState::InGame)))
            .add_system(despawn_player.in_schedule(OnExit(AppState::InGame)))
            .add_systems(
                (
                    player_movement.in_set(PlayerSystemSet::Movement),
                    confine_player_movement.in_set(PlayerSystemSet::Confinement),
                )
                    .in_set(OnUpdate(AppState::InGame))
                    .in_set(OnUpdate(GameState::Running)),
            )
            .add_systems(
                (player_hit_star, enemy_hit_player)
                    .in_set(OnUpdate(AppState::InGame))
                    .in_set(OnUpdate(GameState::Running)),
            );
    }
}
