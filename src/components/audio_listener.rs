use bevy::math::Vec3;
use bevy::prelude::{Component, GlobalTransform, Query, Res, With};

use crate::attributes_3d::attributes3d;
use crate::components::velocity::Velocity;
use crate::fmod_studio::FmodStudio;

#[derive(Component, Default)]
pub struct AudioListener;

impl AudioListener {
    pub(crate) fn update_3d_attributes(
        query: Query<(&GlobalTransform, Option<&Velocity>), With<AudioListener>>,
        studio: Res<FmodStudio>,
    ) {
        if let Ok((transform, velocity)) = query.get_single() {
            let mut velo = Vec3::ZERO;

            if let Some(vel) = velocity {
                velo = vel.current_velocity;
            }

            studio
                .0
                .set_listener_attributes(
                    0,
                    attributes3d(
                        transform.translation(),
                        velo,
                        transform.forward(),
                        transform.up(),
                    ),
                    None,
                )
                .unwrap();
        }
    }
}
