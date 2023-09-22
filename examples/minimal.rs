//! This example demonstrates how to use the FmodPlugin to play a sound.
//! Make sure to follow the instructions in the README.md to set up the demo project.

use bevy::app::PostStartup;
use bevy::prelude::{App, AudioSinkPlayback, Commands, Component, Query, Res, Startup, With};
use bevy::DefaultPlugins;

use bevy_fmod::components::audio_source::AudioSource;
use bevy_fmod::fmod_plugin::FmodPlugin;
use bevy_fmod::fmod_studio::FmodStudio;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FmodPlugin {
                audio_banks_paths: &[
                    "./demo_project/Build/Desktop/Master.bank",
                    "./demo_project/Build/Desktop/Master.strings.bank",
                ],
            },
        ))
        .add_systems(Startup, startup)
        .add_systems(PostStartup, play_music)
        .run();
}

#[derive(Component)]
struct MyMusicPlayer;

fn startup(mut commands: Commands, studio: Res<FmodStudio>) {
    let event_description = studio.0.get_event("event:/Return").unwrap();

    commands
        .spawn(MyMusicPlayer)
        .insert(AudioSource::new(event_description));
}

fn play_music(mut audio_sources: Query<&AudioSource, With<MyMusicPlayer>>) {
    audio_sources.single_mut().play();
}
