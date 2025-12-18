use bevy::ecs::error::Result;
use bevy::ecs::lifecycle::Remove;
use bevy::ecs::observer::On;
use bevy::ecs::system::Query;
use bevy::log::error;
use bevy::prelude::{App, Plugin, PostUpdate, Res, Update};

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
        let studio_instance = match FmodStudio::new(self.audio_banks_paths, self.plugin_paths) {
            Ok(instance) => instance,
            Err(e) => {
                error!("Could not create FMOD studio API instance: {e}");
                return;
            }
        };

        app.add_plugins(VelocityPlugin)
            .insert_resource(studio_instance)
            .add_systems(
                Update,
                (
                    AudioSource::update_3d_attributes,
                    AudioListener::update_3d_attributes,
                ),
            )
            .add_systems(PostUpdate, Self::update)
            .add_observer(on_remove_audio_source);
    }
}

impl FmodPlugin {
    fn update(studio: Res<FmodStudio>) -> Result {
        studio.update()?;
        Ok(())
    }

    #[must_use]
    pub fn new(audio_banks_paths: &'static [&'static str]) -> Self {
        FmodPlugin {
            audio_banks_paths,
            plugin_paths: None,
        }
    }
}

fn on_remove_audio_source(
    remove: On<Remove, AudioSource>,
    query: Query<&mut AudioSource>,
) -> Result {
    let audio_source = query.get(remove.entity)?;
    let event_instance = audio_source.event_instance;

    event_instance.stop(audio_source.despawn_stop_mode)?;
    event_instance.release()?;

    Ok(())
}
