//! This example demonstrates how to use the FmodPlugin to play a sound.
//! Make sure to follow the instructions in the README.md to set up the demo project.

use bevy::app::{App, Startup};
use bevy::prelude::Res;
use bevy::DefaultPlugins;
use bevy_fmod::{FmodPlugin, OneShotPlayer};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FmodPlugin {
                audio_banks_directory: "./demo_project/Build/Desktop",
            },
        ))
        .add_systems(Startup, play_music)
        .run();
}

fn play_music(one_shot_player: Res<OneShotPlayer>) {
    one_shot_player.play_event("event:/return");
}
