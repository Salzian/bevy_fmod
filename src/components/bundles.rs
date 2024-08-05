use crate::prelude::{AudioListener, AudioSource, Velocity};
use bevy::prelude::{Bundle, TransformBundle};
use libfmod::EventDescription;

#[derive(Bundle)]
pub struct SpatialAudioBundle {
    audio_source: AudioSource,
    velocity: Velocity,
    transform: TransformBundle,
}

impl SpatialAudioBundle {
    pub fn new(event_description: EventDescription) -> Self {
        SpatialAudioBundle {
            audio_source: AudioSource::new(event_description),
            velocity: Velocity::default(),
            transform: TransformBundle::default(),
        }
    }
}

#[derive(Bundle, Default)]
pub struct SpatialListenerBundle {
    audio_listener: AudioListener,
    velocity: Velocity,
    transform: TransformBundle,
}
