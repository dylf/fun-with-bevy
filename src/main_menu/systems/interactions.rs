use bevy::{app::AppExit, prelude::*};

use crate::{
    main_menu::{components::*, styles::*},
    AppState,
};

pub fn interact_with_play_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = BUTTON_COLOR_PRESSED.into();
                next_state.set(AppState::InGame);
            }
            Interaction::Hovered => {
                *background_color = BUTTON_COLOR_HOVER.into();
            }
            Interaction::None => {
                *background_color = BUTTON_COLOR_PRIMARY.into();
            }
        }
    }
}

pub fn interact_with_quit_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = BUTTON_COLOR_PRESSED.into();
                app_exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => {
                *background_color = BUTTON_COLOR_HOVER.into();
            }
            Interaction::None => {
                *background_color = BUTTON_COLOR_PRIMARY.into();
            }
        }
    }
}
