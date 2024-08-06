use bevy::math::Vec3;
use bevy::prelude::{Component, Deref, DerefMut, GlobalTransform, Query};
use libfmod::{EventDescription, EventInstance, StopMode};

use crate::attributes_3d::attributes3d;
use crate::components::velocity::Velocity;

/// See the [`Velocity`] component for information on enabling the Doppler effect.
#[derive(Component, Deref, DerefMut)]
pub struct AudioSource {
    #[deref]
    pub event_instance: EventInstance,
    pub stop_mode: Option<StopMode>,
}

impl AudioSource {
    /// Without a `stop_mode` the event will keep playing to the end if the entity is despawned.
    pub fn new(event_description: EventDescription, stop_mode: Option<StopMode>) -> Self {
        Self {
            event_instance: event_description.create_instance().unwrap(),
            stop_mode,
        }
    }

    pub(crate) fn update_3d_attributes(
        mut query: Query<(&AudioSource, &GlobalTransform, Option<&Velocity>)>,
    ) {
        query
            .iter_mut()
            .for_each(|(audio_source, transform, vel_component)| {
                let mut velocity = Vec3::ZERO;

                if let Some(vel_component) = vel_component {
                    velocity = vel_component.current_velocity;
                }

                audio_source
                    .set_3d_attributes(attributes3d(
                        transform.translation(),
                        velocity,
                        *transform.forward(),
                        *transform.up(),
                    ))
                    .unwrap();
            });
    }
}

impl AudioSource {
    #[deprecated = "Use `AudioSource::get_volume` instead."]
    pub fn volume(&self) -> f32 {
        self.get_volume().unwrap().0
    }

    #[deprecated = "Use `AudioSource::set_volume` instead."]
    pub fn set_volume(&self, volume: f32) {
        self.event_instance.set_volume(volume).unwrap();
    }

    #[deprecated = "Use `AudioSource::get_pitch` instead."]
    pub fn speed(&self) -> f32 {
        self.get_pitch().unwrap().0
    }

    #[deprecated = "Use `AudioSource::set_pitch` instead."]
    pub fn set_speed(&self, speed: f32) {
        self.set_pitch(speed).unwrap();
    }

    #[deprecated = "Use `AudioSource::start` instead."]
    pub fn play(&self) {
        if self.get_paused().unwrap() {
            self.set_paused(false).unwrap();
        } else {
            self.start().unwrap();
        }
    }

    #[deprecated = "Use `AudioSource::set_paused(bool)` instead."]
    pub fn pause(&self) {
        self.set_paused(true).unwrap();
    }

    #[deprecated = "Use `AudioSource::get_paused` instead."]
    pub fn is_paused(&self) -> bool {
        self.get_paused().unwrap()
    }

    pub fn toggle(&self) {
        self.set_paused(!self.get_paused().unwrap()).unwrap();
    }
}

impl Drop for AudioSource {
    fn drop(&mut self) {
        if let Some(stop_mode) = self.stop_mode {
            self.stop(stop_mode).unwrap();
        }

        self.release().unwrap();
    }
}
