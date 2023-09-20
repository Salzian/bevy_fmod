use crate::audio_player::attributes_3d;
use crate::EventInstance;
use crate::{AudioListener, AudioPlayer, AudioSource};
use bevy::app::{App, Plugin, PostUpdate, Update};
use bevy::math::Vec3;
use bevy::prelude::{Added, Commands, Entity, GlobalTransform, Query, Res};
use bevy::time::Time;
use bevy_mod_sysfail::sysfail;

use crate::fmod_studio::FmodStudio;

pub struct FmodPlugin {
    pub audio_banks_directory: &'static str,
}

impl Plugin for FmodPlugin {
    fn build(&self, app: &mut App) {
        let res = FmodStudio::new(self.audio_banks_directory);
        app.insert_resource(res)
            .add_systems(Update, (Self::check_for_new_sources, Self::update_sources))
            .add_systems(PostUpdate, Self::update);
    }
}

impl FmodPlugin {
    #[sysfail(log(level = "error"))]
    fn check_for_new_sources(
        mut commands: Commands,
        query: Query<(Entity, &AudioSource), Added<AudioSource>>,
        studio: Res<FmodStudio>,
    ) -> anyhow::Result<()> {
        for (ent, source) in query.iter() {
            let event_description = studio.0.get_event(source.name)?;
            let instance = event_description.create_instance()?;

            // Start the effect already
            instance.start()?;

            commands.entity(ent).insert(AudioPlayer {
                fmod_event: EventInstance(instance),
                previous_position: Vec3::ZERO,
            });
        }

        Ok(())
    }

    #[sysfail(log(level = "error"))]
    fn update_sources(
        mut query: Query<(&GlobalTransform, &mut AudioPlayer)>,
        time: Res<Time>,
    ) -> anyhow::Result<()> {
        for (transform, mut source) in query.iter_mut() {
            let pos = transform.translation();
            let fwd = transform.forward();
            let up = transform.up();

            let delta = pos - source.previous_position;
            let vel = delta / time.delta_seconds();
            source.previous_position = pos;

            let source_attributes = attributes_3d::attributes3d(pos, vel, fwd, up);

            source.fmod_event.0.set_3d_attributes(source_attributes)?;
        }

        Ok(())
    }

    #[sysfail(log(level = "error"))]
    fn update(
        studio: Res<FmodStudio>,
        mut query: Query<(&GlobalTransform, &mut AudioListener)>,
        time: Res<Time>,
    ) -> anyhow::Result<()> {
        if let Ok((transform, mut listener)) = query.get_single_mut() {
            let pos = transform.translation();
            let fwd = transform.forward();
            let up = transform.up();

            let delta = pos - listener.previous_position;
            let vel = delta / time.delta_seconds();
            listener.previous_position = pos;

            let listener_attributes = attributes_3d::attributes3d(pos, vel, fwd, up);

            studio
                .0
                .set_listener_attributes(0, listener_attributes, None)?;
        }

        studio.0.update()?;
        Ok(())
    }
}
