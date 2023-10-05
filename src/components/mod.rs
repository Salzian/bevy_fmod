use crate::prelude::{AudioListener, AudioSource, Velocity};
use bevy::prelude::Bundle;
use libfmod::EventDescription;

pub mod audio_listener;
pub mod audio_source;
pub mod velocity;

#[derive(Bundle)]
pub struct SpatialAudioBundle {
    audio_source: AudioSource,
    velocity: Velocity,
}

impl SpatialAudioBundle {
    pub fn new(event_description: EventDescription) -> Self {
        SpatialAudioBundle {
            audio_source: AudioSource::new(event_description),
            velocity: Velocity::default(),
        }
    }
}

#[derive(Bundle)]
pub struct SpatialListenerBundle {
    audio_listener: AudioListener,
    velocity: Velocity,
}

impl Default for SpatialListenerBundle {
    fn default() -> Self {
        SpatialListenerBundle {
            audio_listener: AudioListener::default(),
            velocity: Velocity::default(),
        }
    }
}
