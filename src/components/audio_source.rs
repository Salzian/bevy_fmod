use bevy::math::Vec3;
use bevy::prelude::{Component, GlobalTransform, Query};
use bevy_mod_sysfail::sysfail;
use libfmod::StopMode::Immediate;
use libfmod::{EventDescription, EventInstance, StopMode};

use crate::attributes_3d::attributes3d;
use crate::components::velocity::Velocity;
use crate::error::Error;
use crate::error::Result;

/// See the [`Velocity`] component for information on enabling the Doppler effect.
#[derive(Component)]
pub struct AudioSource {
    pub event_instance: EventInstance,
}

impl AudioSource {
    pub fn new(event_description: EventDescription) -> Result<AudioSource> {
        Ok(Self {
            event_instance: event_description.create_instance()?,
        })
    }

    #[sysfail(log)]
    pub(crate) fn update_3d_attributes(
        mut query: Query<(&AudioSource, &GlobalTransform, Option<&Velocity>)>,
    ) -> Result<()> {
        for (audio_source, global_transform, velocity_component_option) in query.iter_mut() {
            let mut velocity = Vec3::ZERO;

            if let Some(velocity_component) = velocity_component_option {
                velocity = velocity_component.current_velocity;
            }

            audio_source
                .event_instance
                .set_3d_attributes(attributes3d(
                    global_transform.translation(),
                    velocity,
                    *global_transform.forward(),
                    *global_transform.up(),
                ))
                .map_err(Error::Fmod)?;
        }

        Ok(())
    }
}

impl AudioSource {
    pub fn volume(&self) -> Result<f32> {
        Ok(self.event_instance.get_volume()?.0)
    }

    pub fn set_volume(&self, volume: f32) -> Result<()> {
        Ok(self.event_instance.set_volume(volume)?)
    }

    pub fn speed(&self) -> Result<f32> {
        Ok(self.event_instance.get_pitch()?.0)
    }

    pub fn set_speed(&self, speed: f32) -> Result<()> {
        Ok(self.event_instance.set_pitch(speed)?)
    }

    pub fn play(&self) -> Result<()> {
        let result = if self.event_instance.get_paused()? {
            self.event_instance.set_paused(false)
        } else {
            self.event_instance.start()
        };

        Ok(result?)
    }

    pub fn pause(&self) -> Result<()> {
        Ok(self.event_instance.set_paused(true)?)
    }

    pub fn is_paused(&self) -> Result<bool> {
        Ok(self.event_instance.get_paused()?)
    }

    pub fn stop(&self) -> Result<()> {
        Ok(self.event_instance.stop(StopMode::AllowFadeout)?)
    }

    pub fn empty(&self) -> bool {
        self.event_instance.is_valid()
    }

    pub fn toggle(&self) -> Result<()> {
        if self.is_paused()? {
            self.play()
        } else {
            self.pause()
        }
    }
}

impl Drop for AudioSource {
    fn drop(&mut self) {
        let _ = self.event_instance.stop(Immediate);
        let _ = self.event_instance.release();
    }
}
