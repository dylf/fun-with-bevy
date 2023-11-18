use bevy::prelude::*;

use crate::AppState;

use self::systems::{interactions::*, layout::*};

mod components;
mod styles;
mod systems;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(main_menu)
            .add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)))
            .add_systems(
                (interact_with_play_button, interact_with_quit_button)
                    .in_set(OnUpdate(AppState::MainMenu)),
            );
    }
}

pub fn main_menu() {
    println!("Hello, world!");
}
