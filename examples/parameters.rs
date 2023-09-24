//! This example demonstrates FMOD parameter usage.
//! See https://www.fmod.com/docs/2.02/studio/parameters-reference.html for more details.
//!
//! Controls:
//! Press arrow Up and Down to change the amount of rain you can hear.
//! Press E and M to change the time to Evening and Morning respectively.
//!
//! Note that because `ignore_seek_speed` is `false` in these examples,
//! it could take a few seconds before you can hear the changes in these sound effects.

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
                    "./assets/Master.bank",
                    "./assets/Master.strings.bank",
                    "./assets/SFX.bank",
                ],
            },
        ))
        .add_systems(Startup, startup)
        .add_systems(PostStartup, play_music)
        .add_systems(Update, (set_rain, set_hour))
        .run();
}

#[derive(Component)]
struct ForestSfxPlayer;

#[derive(Component)]
struct CountrySfxPlayer;

fn startup(mut commands: Commands, studio: Res<FmodStudio>) {
    let event_description = studio.0.get_event("event:/Ambience/Forest").unwrap();

    commands
        .spawn(ForestSfxPlayer)
        .insert(AudioSource::new(event_description));

    let event_description = studio.0.get_event("event:/Ambience/Country").unwrap();

    commands
        .spawn(CountrySfxPlayer)
        .insert(AudioSource::new(event_description));
}

fn play_music(audio_sources: Query<&AudioSource>) {
    for audio_source in audio_sources.iter() {
        audio_source.play();
    }
}

fn set_rain(audio_sources: Query<&AudioSource, With<ForestSfxPlayer>>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Up) {
        for audio_source in audio_sources.iter() {
            audio_source
                .event_instance
                .set_parameter_by_name("Rain", 1.0, false)
                .expect("Could not set parameter.");
        }
    }

    if input.just_pressed(KeyCode::Down) {
        for audio_source in audio_sources.iter() {
            audio_source
                .event_instance
                .set_parameter_by_name("Rain", 0.0, false)
                .expect("Could not set parameter.");
        }
    }
}

fn set_hour(
    audio_sources: Query<&AudioSource, With<CountrySfxPlayer>>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::E) {
        for audio_source in audio_sources.iter() {
            audio_source
                .event_instance
                .set_parameter_by_name_with_label("Hour", "Evening", false)
                .expect("Could not set parameter.");
        }
    }

    if input.just_pressed(KeyCode::M) {
        for audio_source in audio_sources.iter() {
            audio_source
                .event_instance
                .set_parameter_by_name_with_label("Hour", "Morning", false)
                .expect("Could not set parameter.");
        }
    }
}
