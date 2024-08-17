//! This example demonstrates how to use the FmodPlugin to play a sound.
//! Make sure to follow the instructions in the README.md to set up the demo project.

use bevy::prelude::*;
use bevy_fmod::prelude::AudioSource;
use bevy_fmod::prelude::*;
use libfmod::StopMode;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FmodPlugin::new(&[
                "./assets/audio/demo_project/Build/Desktop/Master.bank",
                "./assets/audio/demo_project/Build/Desktop/Master.strings.bank",
                "./assets/audio/demo_project/Build/Desktop/Music.bank",
            ]),
        ))
        .add_systems(Startup, startup)
        .add_systems(PostStartup, play_music)
        .run();
}

#[derive(Component)]
struct MyMusicPlayer;

fn startup(mut commands: Commands, studio: Res<FmodStudio>) {
    let event_description = studio.get_event("event:/Music/Level 03").unwrap();

    commands.spawn(MyMusicPlayer).insert(AudioSource {
        event_instance: event_description.create_instance().unwrap(),
        despawn_stop_mode: StopMode::AllowFadeout,
    });
}

fn play_music(mut audio_sources: Query<&AudioSource, With<MyMusicPlayer>>) {
    audio_sources.single_mut().start().unwrap();
}
