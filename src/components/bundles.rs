use crate::prelude::{AudioListener, AudioSource, Velocity};
use bevy::prelude::{Bundle, Transform};

#[derive(Bundle)]
pub struct SpatialAudioBundle {
    audio_source: AudioSource,
    velocity: Velocity,
    transform: Transform,
}

impl SpatialAudioBundle {
    pub fn new(audio_source: AudioSource) -> Self {
        SpatialAudioBundle {
            audio_source,
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
