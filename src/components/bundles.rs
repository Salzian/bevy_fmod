use crate::prelude::{AudioListener, AudioSource, Velocity};
use bevy::prelude::{Bundle, Transform};
use libfmod::{EventDescription, StopMode};

#[derive(Bundle)]
pub struct SpatialAudioBundle {
    audio_source: AudioSource,
    velocity: Velocity,
    transform: Transform,
}

impl SpatialAudioBundle {
    pub fn new(event_description: EventDescription, stop_mode: Option<StopMode>) -> Self {
        SpatialAudioBundle {
            audio_source: AudioSource::new(event_description, stop_mode),
            velocity: Velocity::default(),
            transform: Transform::default(),
        }
    }
}

#[derive(Bundle, Default)]
pub struct SpatialListenerBundle {
    audio_listener: AudioListener,
    velocity: Velocity,
    transform: Transform,
}
