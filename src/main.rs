use bevy::prelude::*;

use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

mod events;
mod game;
mod main_menu;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .add_startup_system(spawn_camera)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .add_system(transition_to_main_menu_state)
        .add_system(transition_to_in_game_state)
        .run();
}

#[derive(States, Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}
