use bevy::prelude::Component;

#[derive(Component)]
pub struct AudioSource {
    pub name: &'static str,
}

impl From<&'static str> for AudioSource {
    fn from(name: &'static str) -> Self {
        AudioSource { name }
    }
}
