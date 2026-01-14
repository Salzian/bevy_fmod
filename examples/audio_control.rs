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
            FmodPlugin::new(&[
                "./assets/audio/demo_project/Build/Desktop/Master.bank",
                "./assets/audio/demo_project/Build/Desktop/Master.strings.bank",
                "./assets/audio/demo_project/Build/Desktop/Music.bank",
            ]),
        ))
        .add_systems(Startup, (startup, display_controls))
        .add_systems(PostStartup, play_music)
        .add_systems(Update, audio_control)
        .run();
}

#[derive(Component)]
struct MyMusicPlayer;

fn startup(mut commands: Commands, studio: Res<FmodStudio>) -> Result {
    let event_description = studio.get_event("event:/Music/Level 03")?;

    commands.spawn(MyMusicPlayer).insert(AudioSource {
        event_instance: event_description.create_instance()?,
        despawn_stop_mode: StopMode::AllowFadeout,
    });

    // In this case only needed to show the controls:
    commands.spawn(Camera2d::default());

    Ok(())
}

fn play_music(audio_source: Single<&AudioSource, With<MyMusicPlayer>>) -> Result {
    audio_source.start()?;
    Ok(())
}

fn audio_control(query: Query<&AudioSource>, input: Res<ButtonInput<KeyCode>>) -> Result {
    if input.just_pressed(KeyCode::KeyS) {
        for audio_player in query.iter() {
            audio_player.stop(StopMode::AllowFadeout)?;
        }
    }

    if input.just_pressed(KeyCode::KeyP) {
        for audio_player in query.iter() {
            audio_player.start()?;
        }
    }

    if input.just_pressed(KeyCode::KeyT) {
        for audio_player in query.iter() {
            audio_player.toggle()?;
        }
    }

    Ok(())
}

fn display_controls(mut commands: Commands) {
    commands.spawn((
        Text::default(),
        children![
            TextSpan::new("Controls: \n"),
            TextSpan::new("S: Stop \n"),
            TextSpan::new("P: Play \n"),
            TextSpan::new("T: Toggle Play/Pause \n"),
        ],
    ));
}
