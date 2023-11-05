//! Audio control:
//! `AudioSource` implements `AudioSinkPlayback` from Bevy, so you can control the audio as normal.
//!
//! Controls:
//! Press S, P and T to stop, play and toggle the sounds, respectively.
//! See `AudioSinkPlayback` documentation for more functions.

use bevy::prelude::*;
use bevy_fmod::prelude::AudioSource;
use bevy_fmod::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FmodPlugin {
                audio_banks_paths: &[
                    "./assets/audio/demo_project/Build/Desktop/Master.bank",
                    "./assets/audio/demo_project/Build/Desktop/Master.strings.bank",
                    "./assets/audio/demo_project/Build/Desktop/Music.bank",
                ],
            },
        ))
        .add_systems(Startup, startup)
        .add_systems(PostStartup, play_music)
        .add_systems(Update, audio_control)
        .run();
}

#[derive(Component)]
struct MyMusicPlayer;

fn startup(mut commands: Commands, studio: Res<FmodStudio>) {
    commands
        .spawn(MyMusicPlayer)
        .insert(studio.build_audio_source("event:/Music/Level 03"));
}

fn play_music(mut audio_sources: Query<&AudioSource, With<MyMusicPlayer>>) {
    audio_sources.single_mut().play();
}

fn audio_control(query: Query<&AudioSource>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::S) {
        for audio_player in query.iter() {
            audio_player.stop();
        }
    }

    if input.just_pressed(KeyCode::P) {
        for audio_player in query.iter() {
            audio_player.play();
        }
    }

    if input.just_pressed(KeyCode::T) {
        for audio_player in query.iter() {
            audio_player.toggle();
        }
    }
}
