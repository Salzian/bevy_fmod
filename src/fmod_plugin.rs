use bevy::app::PreStartup;
use bevy::log::error;
use bevy::prelude::{App, Plugin, PostUpdate, Res, Update, World};

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
            .add_systems(PreStartup, register_component_hooks)
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
    fn update(studio: Res<FmodStudio>) {
        studio
            .update()
            .unwrap_or_else(|e| error!("Failed to tick FMOD Studio: {}", e));
    }

    #[must_use]
    pub fn new(audio_banks_paths: &'static [&'static str]) -> Self {
        FmodPlugin {
            audio_banks_paths,
            plugin_paths: None,
        }
    }
}

fn register_component_hooks(world: &mut World) {
    world
        .register_component_hooks::<AudioSource>()
        .on_remove(|mut world, hook_context| {
            let entity = hook_context.entity;
            let mut entity_mut = world.entity_mut(entity);
            let audio_source = entity_mut.get_mut::<AudioSource>().unwrap();
            let event_instance = audio_source.event_instance;

            event_instance.stop(audio_source.despawn_stop_mode).unwrap();
            event_instance.release().unwrap();
        });
}
