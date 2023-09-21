use bevy::prelude::{App, Plugin, PostUpdate, Res, Update};
use bevy_mod_sysfail::sysfail;

use crate::components::audio_listener::AudioListener;
use crate::components::audio_source::AudioSource;
use crate::components::velocity::VelocityPlugin;
use crate::fmod_studio::FmodStudio;

pub struct FmodPlugin {
    pub audio_banks_paths: &'static [&'static str],
}

impl Plugin for FmodPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VelocityPlugin)
            .insert_resource(FmodStudio::new(self.audio_banks_paths))
            .add_systems(
                Update,
                (
                    AudioSource::update_3d_attributes,
                    AudioListener::update_3d_attributes,
                ),
            )
            .add_systems(PostUpdate, Self::update);
    }
}

impl FmodPlugin {
    #[sysfail(log(level = "error"))]
    fn update(studio: Res<FmodStudio>) -> anyhow::Result<()> {
        studio.0.update()?;
        Ok(())
    }
}
