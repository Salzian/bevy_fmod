use bevy::prelude::{App, Plugin, PostUpdate, Res, Update};
use bevy_mod_sysfail::sysfail;

use crate::components::audio_listener::AudioListener;
use crate::components::audio_source::AudioSource;
use crate::components::velocity::VelocityPlugin;
use crate::fmod_studio::FmodStudio;

/// Initializes the FMOD Studio API and provides systems to update the audio sources and listeners.
pub struct FmodPlugin {
    /// Paths to the audio banks which are usually in the Build folder of the FMOD project.
    pub audio_banks_paths: &'static [&'static str],

    /// Optionally you can provide paths to FMOD plugins which will then be loaded automatically.
    /// For more information see: <https://www.fmod.com/docs/2.01/api/core-guide.html#dynamic>
    pub plugin_paths: Option<&'static [&'static str]>,
}

impl Plugin for FmodPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VelocityPlugin)
            .insert_resource(FmodStudio::new(self.audio_banks_paths, self.plugin_paths))
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

    pub fn new(audio_banks_paths: &'static [&'static str]) -> Self {
        FmodPlugin {
            audio_banks_paths,
            plugin_paths: None,
        }
    }
}
