use bevy::prelude::{Component, Vec3};

#[derive(Component)]
pub struct AudioListener {
    pub previous_position: Vec3,
}

impl Default for AudioListener {
    fn default() -> Self {
        AudioListener {
            previous_position: Vec3::default(),
        }
    }
}
