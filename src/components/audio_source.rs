use crate::attributes_3d::attributes3d;
use crate::components::velocity::Velocity;
use bevy::ecs::error::Result;
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
    ) -> Result {
        for (audio_source, transform, vel_component) in query.iter_mut() {
            let mut velocity = Vec3::ZERO;

            if let Some(vel_component) = vel_component {
                velocity = vel_component.current_velocity;
            }

            audio_source.set_3d_attributes(attributes3d(
                transform.translation(),
                velocity,
                *transform.forward(),
                *transform.up(),
            ))?;
        }

        Ok(())
    }

    pub fn toggle(&self) {
        self.event_instance
            .set_paused(!self.event_instance.get_paused().unwrap())
            .unwrap();
    }
}
