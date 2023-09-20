//! This example demonstrates how to use the FmodPlugin to play a sound.
//! Make sure to follow the instructions in the README.md to set up the demo project.

use bevy::app::{App, Startup};
use bevy::prelude::Commands;
use bevy::DefaultPlugins;
use bevy_fmod::{AudioSource, FmodPlugin};

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

fn play_music(mut commands: Commands) {
    commands.spawn(AudioSource::from("event:/return"));
}
