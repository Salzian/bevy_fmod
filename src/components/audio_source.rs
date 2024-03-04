use bevy::math::Vec3;
use bevy::prelude::{AudioSinkPlayback, Component, Deref, DerefMut, GlobalTransform, Query};
use libfmod::StopMode::Immediate;
use libfmod::{EventDescription, EventInstance, StopMode};

use crate::attributes_3d::attributes3d;
use crate::components::velocity::Velocity;

/// See the [`Velocity`] component for information on enabling the Doppler effect.
#[derive(Component, Deref, DerefMut)]
pub struct AudioSource(EventInstance);

impl AudioSource {
    pub fn new(event_description: EventDescription) -> Self {
        Self(event_description.create_instance().unwrap())
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
                        transform.forward(),
                        transform.up(),
                    ))
                    .unwrap();
            });
    }
}

impl AudioSinkPlayback for AudioSource {
    fn volume(&self) -> f32 {
        self.get_volume().unwrap().0
    }

    fn set_volume(&self, volume: f32) {
        self.0.set_volume(volume).unwrap();
    }

    fn speed(&self) -> f32 {
        self.get_pitch().unwrap().0
    }

    fn set_speed(&self, speed: f32) {
        self.set_pitch(speed).unwrap();
    }

    fn play(&self) {
        if self.get_paused().unwrap() {
            self.set_paused(false).unwrap();
        } else {
            self.start().unwrap();
        }
    }

    fn pause(&self) {
        self.set_paused(true).unwrap();
    }

    fn is_paused(&self) -> bool {
        self.get_paused().unwrap()
    }

    fn stop(&self) {
        self.0.stop(StopMode::AllowFadeout).unwrap();
    }

    fn empty(&self) -> bool {
        self.is_valid()
    }
}

impl Drop for AudioSource {
    fn drop(&mut self) {
        self.0.stop(Immediate).unwrap();
        self.release().unwrap();
    }
}
