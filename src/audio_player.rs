pub mod audio_listener;
pub mod audio_source;

use crate::EventInstance;
use bevy::prelude::{AudioSinkPlayback, Component, Vec3};

#[derive(Component)]
pub struct AudioPlayer {
    pub fmod_event: EventInstance,
    pub previous_position: Vec3,
}

impl AudioSinkPlayback for AudioPlayer {
    fn volume(&self) -> f32 {
        let (volume, _final_volume) = self.fmod_event.0.get_volume().unwrap();
        volume
    }

    fn set_volume(&self, volume: f32) {
        self.fmod_event.0.set_volume(volume).unwrap();
    }

    /// Gets the pitch
    fn speed(&self) -> f32 {
        let (pitch, _final_pitch) = self.fmod_event.0.get_pitch().unwrap();
        pitch
    }

    /// Sets the pitch:
    /// "The pitch multiplier is used to modulate the event instance's pitch.
    /// The pitch multiplier can be set to any value greater than or equal to zero but
    /// the final combined pitch is clamped to the range [0.0, 100.0] before being applied."
    fn set_speed(&self, speed: f32) {
        self.fmod_event.0.set_pitch(speed).unwrap();
    }

    fn play(&self) {
        // AudioSinkPlayback does not have a resume function so we go for this
        if self.is_paused() {
            self.fmod_event.0.set_paused(false).unwrap();
        } else {
            self.fmod_event.0.start().unwrap();
        }
    }

    fn pause(&self) {
        self.fmod_event.0.set_paused(true).unwrap();
    }

    fn is_paused(&self) -> bool {
        self.fmod_event.0.get_paused().unwrap()
    }

    fn stop(&self) {
        // Todo: configurable StopMode
        self.fmod_event
            .0
            .stop(libfmod::StopMode::AllowFadeout)
            .unwrap();
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
