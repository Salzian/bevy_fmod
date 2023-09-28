use bevy::math::Vec3;
use bevy::prelude::{AudioSinkPlayback, Component, GlobalTransform, Query};
use libfmod::StopMode::Immediate;
use libfmod::{EventDescription, EventInstance, StopMode};

use crate::attributes_3d::attributes3d;
use crate::components::velocity::Velocity;

#[derive(Component)]
pub struct AudioSource {
    pub event_instance: EventInstance,
}

impl AudioSource {
    pub fn new(event_description: EventDescription) -> Self {
        Self {
            event_instance: event_description.create_instance().unwrap(),
        }
    }

    pub(crate) fn update_3d_attributes(
        mut query: Query<(&AudioSource, &GlobalTransform, Option<&Velocity>)>,
    ) {
        query
            .iter_mut()
            .for_each(|(audio_source, transform, velocity)| {
                let mut velo = Vec3::ZERO;

                if let Some(vel) = velocity {
                    velo = vel.current_velocity;
                }

                audio_source
                    .event_instance
                    .set_3d_attributes(attributes3d(
                        transform.translation(),
                        velo,
                        transform.forward(),
                        transform.up(),
                    ))
                    .unwrap();
            });
    }
}

impl AudioSinkPlayback for AudioSource {
    fn volume(&self) -> f32 {
        self.event_instance.get_volume().unwrap().0
    }

    fn set_volume(&self, volume: f32) {
        self.event_instance.set_volume(volume).unwrap();
    }

    fn speed(&self) -> f32 {
        self.event_instance.get_pitch().unwrap().0
    }

    fn set_speed(&self, speed: f32) {
        self.event_instance.set_pitch(speed).unwrap();
    }

    fn play(&self) {
        self.event_instance.start().unwrap();
    }

    fn pause(&self) {
        self.event_instance.set_paused(true).unwrap();
    }

    fn is_paused(&self) -> bool {
        self.event_instance.get_paused().unwrap()
    }

    fn stop(&self) {
        self.event_instance.stop(StopMode::AllowFadeout).unwrap();
    }

    fn empty(&self) -> bool {
        self.event_instance.is_valid()
    }
}

impl Drop for AudioSource {
    fn drop(&mut self) {
        self.event_instance.stop(Immediate).unwrap();
        self.event_instance.release().unwrap();
    }
}
