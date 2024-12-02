//! [`Bundle`]s for spatial audio components in a Bevy application.
//!
//! For more information on bundles, see the [`Bundle`] trait.

use crate::prelude::{AudioListener, AudioSource, Velocity};
use bevy::prelude::{Bundle, Transform};
use libfmod::{EventDescription, StopMode};

/// A bundle that includes all components required for emitting spatial audio.
#[derive(Bundle)]
pub struct SpatialAudioBundle {
    audio_source: AudioSource,
    velocity: Velocity,
    transform: Transform,
}

impl SpatialAudioBundle {
    /// Creates a new `SpatialAudioBundle` from the given `EventDescription`.
    ///
    /// # Arguments
    ///
    /// * `event_description` - An [`EventDescription`] that provides the necessary information to
    ///   create an [`AudioSource`].
    ///
    /// # Returns
    ///
    /// A new instance of [`SpatialAudioBundle`] containing the components required for emitting
    /// spatial audio.
    pub fn new(event_description: EventDescription) -> Self {
        SpatialAudioBundle {
            audio_source: AudioSource {
                event_instance: event_description.create_instance().unwrap(),
                despawn_stop_mode: StopMode::AllowFadeout,
            },
            velocity: Velocity::default(),
            transform: Transform::default(),
        }
    }
}

impl From<AudioSource> for SpatialAudioBundle {
    fn from(value: AudioSource) -> Self {
        SpatialAudioBundle {
            audio_source: value,
            velocity: Velocity::default(),
            transform: Transform::default(),
        }
    }
}

/// A bundle that includes all components required for listening to spatial audio.
#[derive(Bundle, Default)]
pub struct SpatialListenerBundle {
    audio_listener: AudioListener,
    velocity: Velocity,
    transform: Transform,
}
