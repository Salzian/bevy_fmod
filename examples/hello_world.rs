extern crate bevy;
extern crate bevy_fmod_core;

use bevy::prelude::EventWriter;
use bevy::DefaultPlugins;
use bevy::{app::App, prelude::Startup};
use bevy_fmod_core::{FmodPlugin, PlaySoundEvent};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FmodPlugin {
                audio_banks_directory: "./examples/demo_project/Build/Desktop",
            },
        ))
        .add_systems(Startup, play_music)
        .run();
}

fn play_music(mut event_writer: EventWriter<PlaySoundEvent>) {
    event_writer.send(PlaySoundEvent("event:/return"));
}
