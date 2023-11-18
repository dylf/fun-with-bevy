use bevy::prelude::*;

use super::GameState;

pub fn toggle_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    state: ResMut<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match state.0 {
            GameState::Paused => {
                next_state.set(GameState::Running);
                println!("Game resumed!");
            }
            GameState::Running => {
                next_state.set(GameState::Paused);
                println!("Game paused!");
            }
        }
    }
}
