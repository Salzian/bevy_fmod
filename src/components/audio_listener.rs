use bevy::math::Vec3;
use bevy::prelude::{Component, GlobalTransform, Query, Res, With};

use crate::attributes_3d::attributes3d;
use crate::components::velocity::Velocity;
use crate::fmod_studio::FmodStudio;

/// See the [`Velocity`] component for information on enabling the Doppler effect.
#[derive(Component, Default)]
pub struct AudioListener;

impl AudioListener {
    pub(crate) fn update_3d_attributes(
        query: Query<(&GlobalTransform, Option<&Velocity>), With<AudioListener>>,
        studio: Res<FmodStudio>,
    ) {
        if let Ok((transform, vel_component)) = query.get_single() {
            let mut velocity = Vec3::ZERO;

            if let Some(vel_component) = vel_component {
                velocity = vel_component.current_velocity;
            }

            println!("{}", transform.translation());
            println!("{}", velocity);
            println!("{}", transform.forward());
            println!("{}", transform.up());
            studio
                .0
                .set_listener_attributes(
                    0,
                    attributes3d(
                        transform.translation(),
                        velocity,
                        transform.forward(),
                        transform.up(),
                    ),
                    None,
                )
                .unwrap();
        }
    }
}
