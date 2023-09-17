use bevy::prelude::EventWriter;
use bevy::DefaultPlugins;
use bevy::{app::App, prelude::Startup};
use bevy_fmod::{FmodPlugin, PlaySoundEvent};

/// This example demonstrates how to use the FmodPlugin to play a sound.
/// Make sure to follow the instructions in the README.md to set up the demo project.
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

fn play_music(mut event_writer: EventWriter<PlaySoundEvent>) {
    event_writer.send(PlaySoundEvent("event:/return"));
}
