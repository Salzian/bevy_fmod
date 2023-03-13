extern crate bevy;
extern crate bevy_fmod;

use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::prelude::EventWriter;
use bevy_fmod::{FmodPlugin, PlaySoundEvent};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FmodPlugin {
            audio_banks_directory: "./examples/demo_project/Build/Desktop",
        })
        .add_startup_system(play_music)
        .run();
}

fn play_music(mut event_writer: EventWriter<PlaySoundEvent>) {
    event_writer.send(PlaySoundEvent("event:/return"));
}
