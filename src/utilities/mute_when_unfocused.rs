use crate::FmodStudio;
use bevy::app::{App, Plugin, Update};
use bevy::ecs::error::Result;
use bevy::prelude::{Entity, Local, MessageReader, ResMut, Single, With};
use bevy::window::{PrimaryWindow, WindowFocused};

/// When this plugin is added, the audio will be muted when the [PrimaryWindow] is not focused
/// and vice versa.
pub struct MuteWhenUnfocused;

impl Plugin for MuteWhenUnfocused {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mute_when_unfocused);
    }
}

fn mute_when_unfocused(
    mut focus_event: MessageReader<WindowFocused>,
    primary_window: Single<Entity, With<PrimaryWindow>>,
    mut last_focus_state: Local<bool>,
    studio: ResMut<FmodStudio>,
) -> Result {
    for WindowFocused { window, focused } in focus_event.read() {
        if *window == *primary_window && *last_focus_state != *focused {
            studio.get_bus("bus:/")?.set_mute(!focused)?;
            *last_focus_state = *focused;
        }
    }

    Ok(())
}
