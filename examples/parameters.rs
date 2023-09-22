//! FMOD local parameter usage
//! Todo: Add example for setting parameter by label

use bevy::prelude::*;

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
        .add_systems(Update, increase_some_parameter)
        .run();
}

#[derive(Component)]
struct MyMusicPlayer;

fn startup(mut commands: Commands, studio: Res<FmodStudio>) {
    let event_description = studio.0.get_event("event:/music").unwrap();

    commands
        .spawn(MyMusicPlayer)
        .insert(AudioSource::new(event_description));
}

fn play_music(audio_sources: Query<&AudioSource, With<MyMusicPlayer>>) {
    audio_sources.single().play();
}

fn increase_some_parameter(
    audio_sources: Query<&AudioSource, With<MyMusicPlayer>>,
    input: Res<Input<KeyCode>>,
    mut value: Local<f32>,
) {
    if input.just_pressed(KeyCode::T) {
        *value = *value + 0.1;
        for audio_source in audio_sources.iter() {
            audio_source.set_parameter("parameter_name", *value, true);
        }
    }
}
