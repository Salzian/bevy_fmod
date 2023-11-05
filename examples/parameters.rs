//! This example demonstrates FMOD parameter usage.
//!
//! Controls:
//! Press arrow Up and Down to change the amount of rain you can hear.
//! Press E and M to change the time to Evening and Morning respectively.
//!
//! Note that because `ignore_seek_speed` is `false` in these examples,
//! it could take a few seconds before you can hear the changes in these sound effects.
//! See https://www.fmod.com/docs/2.02/studio/parameters-reference.html for more details.
//!
//! # Global parameters
//!
//! The FMOD example project unfortunately does not have any global parameters, so we can't include
//! a runnable example of that here.
//!
//! ## Usage
//!
//! ```rust
//! fn main() {
//!     App::new()
//!         .add_plugins((
//!             DefaultPlugins,
//!             FmodPlugin {
//!                 audio_banks_paths: &["<path to your audio banks>"],
//!             },
//!         ))
//!         // ...
//!         .add_systems(Update, (set_some_global_parameter))
//!         .run();
//! }
//!
//! fn set_some_global_parameter(studio: Res<FmodStudio>) {
//!    studio.0.set_parameter_by_name("SomeGlobalParameter", 0.5, false).unwrap();
//! }
//! ```

use bevy::prelude::*;
use bevy_fmod::prelude::AudioSource;
use bevy_fmod::prelude::*;

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
    commands
        .spawn(ForestSfxPlayer)
        .insert(studio.build_audio_source("event:/Ambience/Forest"));

    commands
        .spawn(CountrySfxPlayer)
        .insert(studio.build_audio_source("event:/Ambience/Country"));
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
