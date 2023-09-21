use bevy::app::{App, FixedUpdate, Plugin};
use bevy::math::Vec3;
use bevy::prelude::{Bundle, Component, FixedTime, GlobalTransform, Query, Res};

#[derive(Bundle, Default)]
pub struct VelocityBundle {
    pub velocity: Velocity,
    pub transform: GlobalTransform,
}

#[derive(Component, Default)]
pub struct Velocity {
    last_position: Vec3,
    pub(crate) current_velocity: Vec3,
}

pub(crate) struct VelocityPlugin;

impl VelocityPlugin {
    fn update_velocity(
        mut velocity: Query<(&mut Velocity, &GlobalTransform)>,
        time: Res<FixedTime>,
    ) {
        velocity.iter_mut().for_each(|(mut velocity, transform)| {
            let current_position = transform.translation();
            let delta_position = current_position - velocity.last_position;
            velocity.current_velocity = delta_position / time.period.as_millis() as f32;
            velocity.last_position = current_position;
        })
    }
}

impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, Self::update_velocity);
    }
}
