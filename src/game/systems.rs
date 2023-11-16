use bevy::prelude::*;

use super::GameState;

pub fn toggle_game_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    state: ResMut<State<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match state.0 {
            GameState::Paused => {
                commands.insert_resource(NextState(Some(GameState::Running)));
                println!("Game resumed!");
            }
            GameState::Running => {
                commands.insert_resource(NextState(Some(GameState::Paused)));
                println!("Game paused!");
            }
        }
    }
}
