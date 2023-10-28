use bevy::prelude::{Component, GlobalTransform, Query, Res, With};

use crate::attributes_3d::attributes3d;
use crate::components::velocity::Velocity;
use crate::fmod_studio::FmodStudio;

#[derive(Component, Default)]
pub struct AudioListener;

impl AudioListener {
    pub(crate) fn update_3d_attributes(
        query: Query<(&Velocity, &GlobalTransform), With<AudioListener>>,
        studio: Res<FmodStudio>,
    ) {
        match query.get_single() {
            Ok((velocity, transform)) => {
                println!("{}", transform.translation());
                println!("{}", velocity.current_velocity);
                println!("{}", transform.forward());
                println!("{}", transform.up());
                studio
                    .0
                    .set_listener_attributes(
                        0,
                        attributes3d(
                            transform.translation(),
                            velocity.current_velocity,
                            transform.forward(),
                            transform.up(),
                        ),
                        None,
                    )
                    .unwrap();
            }
            _ => {}
        }
    }
}
