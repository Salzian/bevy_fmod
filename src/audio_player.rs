pub mod attributes_3d;
pub mod audio_listener;
pub mod audio_source;

use crate::EventInstance;
use bevy::audio::AudioSinkPlayback;
use bevy::prelude::{Component, Vec3};

#[derive(Component)]
pub struct AudioPlayer {
    pub fmod_event: EventInstance,
    pub previous_position: Vec3,
}

impl AudioSinkPlayback for AudioPlayer {
    fn volume(&self) -> f32 {
        let (volume, _final_volume) = self
            .fmod_event
            .0
            .get_volume()
            .expect("Failed to get volume.");
        volume
    }

    fn set_volume(&self, volume: f32) {
        self.fmod_event
            .0
            .set_volume(volume)
            .expect("Failed to set volume.");
    }

    fn speed(&self) -> f32 {
        let (pitch, _final_pitch) = self
            .fmod_event
            .0
            .get_pitch()
            .expect("Failed to get pitch (speed).");
        pitch
    }

    /// Sets the pitch:
    /// "The pitch multiplier is used to modulate the event instance's pitch.
    /// The pitch multiplier can be set to any value greater than or equal to zero but
    /// the final combined pitch is clamped to the range [0.0, 100.0] before being applied."
    fn set_speed(&self, speed: f32) {
        self.fmod_event
            .0
            .set_pitch(speed)
            .expect("Failed to set pitch (speed).");
    }

    fn play(&self) {
        // AudioSinkPlayback does not have a resume function so we go for this
        if self.is_paused() {
            self.fmod_event
                .0
                .set_paused(false)
                .expect("Failed to set paused state.");
        } else {
            self.fmod_event.0.start().expect("Failed to start event.");
        }
    }

    fn pause(&self) {
        self.fmod_event
            .0
            .set_paused(true)
            .expect("Failed to set paused state.");
    }

    fn is_paused(&self) -> bool {
        self.fmod_event
            .0
            .get_paused()
            .expect("Failed to get paused state.")
    }

    fn stop(&self) {
        // Todo: configurable StopMode
        self.fmod_event
            .0
            .stop(libfmod::StopMode::AllowFadeout)
            .expect("Failed to stop event,");
    }

    fn empty(&self) -> bool {
        !self.fmod_event.0.is_valid()
    }
}

impl Drop for AudioPlayer {
    fn drop(&mut self) {
        self.fmod_event
            .0
            .release()
            .expect("Error releasing FMOD event instance");
    }
}
