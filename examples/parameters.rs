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
            FmodPlugin::new(&[
                "./assets/audio/demo_project/Build/Desktop/Master.bank",
                "./assets/audio/demo_project/Build/Desktop/Master.strings.bank",
                "./assets/audio/demo_project/Build/Desktop/SFX.bank",
            ]),
        ))
        .add_systems(Startup, (startup, display_controls))
        .add_systems(PostStartup, play_music)
        .add_systems(Update, (set_rain, set_hour))
        .run();
}

#[derive(Component)]
struct ForestSfxPlayer;

#[derive(Component)]
struct CountrySfxPlayer;

fn startup(mut commands: Commands, studio: Res<FmodStudio>) {
    let event_description = studio.get_event("event:/Ambience/Forest").unwrap();

    commands.spawn(ForestSfxPlayer).insert(AudioSource {
        event_instance: event_description.create_instance().unwrap(),
        despawn_stop_mode: StopMode::AllowFadeout,
    });

    let event_description = studio.get_event("event:/Ambience/Country").unwrap();

    commands.spawn(CountrySfxPlayer).insert(AudioSource {
        event_instance: event_description.create_instance().unwrap(),
        despawn_stop_mode: StopMode::AllowFadeout,
    });

    // In this case only needed to show the controls:
    commands.spawn(Camera2d::default());
}

fn play_music(audio_sources: Query<&AudioSource>) {
    for audio_source in audio_sources.iter() {
        audio_source.start().unwrap();
    }
}

fn set_rain(
    audio_sources: Query<&AudioSource, With<ForestSfxPlayer>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::ArrowUp) {
        for audio_source in audio_sources.iter() {
            audio_source
                .set_parameter_by_name("Rain", 1.0, false)
                .expect("Could not set parameter.");
        }
    }

    if input.just_pressed(KeyCode::ArrowDown) {
        for audio_source in audio_sources.iter() {
            audio_source
                .set_parameter_by_name("Rain", 0.0, false)
                .expect("Could not set parameter.");
        }
    }
}

fn set_hour(
    audio_sources: Query<&AudioSource, With<CountrySfxPlayer>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyE) {
        for audio_source in audio_sources.iter() {
            audio_source
                .set_parameter_by_name_with_label("Hour", "Evening", false)
                .expect("Could not set parameter.");
        }
    }

    if input.just_pressed(KeyCode::KeyM) {
        for audio_source in audio_sources.iter() {
            audio_source
                .set_parameter_by_name_with_label("Hour", "Morning", false)
                .expect("Could not set parameter.");
        }
    }
}

fn display_controls(mut commands: Commands) {
    let text_entity = commands.spawn(Text::default()).id();
    commands.entity(text_entity).with_children(|parent| {
        parent.spawn(TextSpan::new("Controls: \n"));
        parent.spawn(TextSpan::new("Arrow Up: Increase Rain \n"));
        parent.spawn(TextSpan::new("Arrow Down: Decrease Rain \n"));
        parent.spawn(TextSpan::new("E: Change time to Evening \n"));
        parent.spawn(TextSpan::new("M: Change time to Morning \n"));
    });
}

