use crate::attributes_3d::attributes3d;
use crate::components::velocity::Velocity;
use bevy::math::Vec3;
use bevy::prelude::{Component, Deref, DerefMut, GlobalTransform, Query};
use libfmod::{EventInstance, StopMode};

/// See the [`Velocity`] component for information on enabling the Doppler effect.
#[derive(Component, Deref, DerefMut)]
pub struct AudioSource {
    /// The [EventInstance] that is playing the audio. Create an instance from an
    /// [EventDescription](libfmod::EventDescription) using
    /// [EventDescription::create_instance](libfmod::EventDescription::create_instance).
    #[deref]
    pub event_instance: EventInstance,
    /// The [StopMode] to use when the entity despawns.
    pub despawn_stop_mode: StopMode,
}

impl AudioSource {
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
        self.event_instance
            .set_paused(!self.event_instance.get_paused().unwrap())
            .unwrap();
    }
}
