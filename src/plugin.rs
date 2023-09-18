use bevy::app::{App, Plugin, PostUpdate, Update};
use bevy::ecs::event::EventReader;
use bevy::ecs::system::Res;
use bevy_mod_sysfail::sysfail;
use libfmod::EventDescription;

use crate::fmod_studio::FmodStudio;
use crate::PlaySoundEvent;

pub struct FmodPlugin {
    pub audio_banks_directory: &'static str,
}

impl Plugin for FmodPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlaySoundEvent>()
            .insert_resource(FmodStudio::new(self.audio_banks_directory))
            .add_systems(Update, Self::play_incoming_events)
            .add_systems(PostUpdate, Self::update);
    }
}

impl FmodPlugin {
    #[sysfail(log(level = "error"))]
    fn update(studio: Res<FmodStudio>) -> anyhow::Result<()> {
        studio.0.update()?;
        Ok(())
    }

    #[sysfail(log(level = "error"))]
    fn play_incoming_events(
        mut events: EventReader<PlaySoundEvent>,
        studio: Res<FmodStudio>,
    ) -> anyhow::Result<()> {
        for event in events.iter() {
            let event_description = studio.0.get_event(event.0)?;
            Self::play_event(&event_description)?;
        }

        Ok(())
    }

    fn play_event(event_description: &EventDescription) -> anyhow::Result<()> {
        let instance = event_description.create_instance()?;

        instance.start()?;
        instance.release()?;

        Ok(())
    }
}
