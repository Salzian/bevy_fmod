use bevy::prelude::{App, Plugin, PostUpdate, Res, Update};
use bevy_mod_sysfail::sysfail;

use crate::components::{AudioListener, AudioSource, VelocityPlugin};
use crate::fmod_studio::FmodStudio;

pub struct FmodPlugin {
    /// Paths to the audio banks which are usually in the Build folder of the FMOD project.
    pub audio_banks_paths: &'static [&'static str],

    /// Optionally you can provide paths to FMOD plugins which will then be loaded automatically.
    /// For more information see: https://www.fmod.com/docs/2.01/api/core-guide.html#dynamic
    pub plugin_paths: Option<&'static [&'static str]>,
}

// If the plugin fails to initialize, there is no point in continuing the application.
#[allow(clippy::expect_used)]
impl Plugin for FmodPlugin {
    /// # Panics
    ///
    /// Panics if the FMOD Studio instance could not be initialized.
    fn build(&self, app: &mut App) {
        let fmod_studio = FmodStudio::new(self.audio_banks_paths, self.plugin_paths)
            .expect("Failed to initialize FMOD Studio");

        app.add_plugins(VelocityPlugin)
            .insert_resource(fmod_studio)
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
