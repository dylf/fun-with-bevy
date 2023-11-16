use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::events::*;
use crate::game::GameState;
use crate::AppState;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn transition_to_in_game_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) && app_state.0 != AppState::InGame {
        commands.insert_resource(NextState(Some(AppState::InGame)));
        println!("Game started!");
    }
}

pub fn transition_to_main_menu_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) && app_state.0 != AppState::MainMenu {
        commands.insert_resource(NextState(Some(AppState::MainMenu)));
        commands.insert_resource(NextState(Some(GameState::Paused)));
        println!("Main menu!");
    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(mut commands: Commands, mut game_over_reader: EventReader<GameOver>) {
    for event in game_over_reader.iter() {
        println!("Game over! Score: {}", event.score);
        commands.insert_resource(NextState(Some(AppState::GameOver)));
        commands.insert_resource(NextState(Some(GameState::Paused)));
    }
}
