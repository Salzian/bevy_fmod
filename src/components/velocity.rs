use bevy::app::{App, Plugin, Update};
use bevy::math::Vec3;
use bevy::prelude::{Component, GlobalTransform, Local, Query, Res, Time};

/// Automatic velocity updates for [`AudioListener`] and [`AudioSource`]
///
/// Make sure to add this component to your listener and source entities in order
/// to enable the Doppler effect. The recommended way to do this is to use the [`SpatialAudioBundle`]
/// and [`SpatialListenerBundle`].
#[derive(Component, Default)]
pub struct Velocity {
    last_position: Vec3,
    pub(crate) current_velocity: Vec3,
}

pub(crate) struct VelocityPlugin;

impl VelocityPlugin {
    fn update_velocity(
        mut velocity: Query<(&mut Velocity, &GlobalTransform)>,
        time: Res<Time>,
        mut last_delta: Local<f32>,
    ) {
        let delta_time = *last_delta;

        velocity.iter_mut().for_each(|(mut velocity, transform)| {
            let current_position = transform.translation();
            let delta_position = current_position - velocity.last_position;

            if delta_time != 0.0 {
                velocity.current_velocity = delta_position / delta_time;
                velocity.last_position = current_position;
            }
        });

        *last_delta = time.delta().as_secs_f32();
    }
}

impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::update_velocity);
    }
}
