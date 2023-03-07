extern crate bevy;
extern crate libfmod;

use bevy::app::{App, CoreStage, Plugin};
use bevy::ecs::event::EventReader;
use bevy::ecs::world::World;
use bevy::log::{debug, warn};
use bevy::prelude::NonSend;

use libfmod::ffi::{FMOD_INIT_NORMAL, FMOD_STUDIO_INIT_NORMAL, FMOD_STUDIO_LOAD_BANK_NORMAL};
use libfmod::Studio;
use play_sound_event::PlaySoundEvent;
use std::fs::read_dir;

pub mod play_sound_event;

pub struct FmodPlugin;

impl Plugin for FmodPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlaySoundEvent>()
            .add_startup_system(startup)
            .add_system_to_stage(CoreStage::PostUpdate, update);
    }
}

fn startup(world: &mut World) {
    let studio = init_studio();
    load_banks(&studio);
    world.insert_non_send_resource(studio);
}

fn update(mut events: EventReader<PlaySoundEvent>, studio: NonSend<Studio>) {
    studio.update().expect("Error while updating FMOD studio");

    for event in events.iter() {
        let event_description = match studio.get_event(event.0) {
            Ok(event) => event,
            Err(error) => {
                warn!("Failed to get event: {}", event.0);
                warn!("Error: {:?}", error);
                continue;
            }
        };

        let instance = event_description
            .create_instance()
            .expect("Failed to create event instance");
        instance.start().expect("Failed to start event");
        instance
            .release()
            .expect("Failed to release event instance");
    }
}

fn init_studio() -> Studio {
    let studio = Studio::create().expect("Failed to create FMOD studio");
    studio
        .initialize(1024, FMOD_STUDIO_INIT_NORMAL, FMOD_INIT_NORMAL, None)
        .expect("Failed to initialize FMOD studio");
    studio
}

fn load_banks(studio: &Studio) {
    // Collect all files within the fmod/Example/Build/Desktop directory
    let files = read_dir("fmod/Example/Build/Desktop")
        .expect("Failed to read directory")
        .map(|res| res.map(|e| e.path()))
        // Remove all Dialogue* files
        .filter(|path| {
            let path = path.as_ref().unwrap();
            let path = path.to_str().unwrap();
            !path.starts_with("Dialogue")
        })
        .collect::<Result<Vec<_>, std::io::Error>>()
        .expect("Failed to collect files");

    // Print all files
    files.iter().for_each(|file| debug!("{:?}", file));

    // Load all files as banks
    for file in files {
        debug!("Loading bank file: {:?}", file);

        match studio.load_bank_file(
            file.to_str().expect("Failed to convert path to string"),
            FMOD_STUDIO_LOAD_BANK_NORMAL,
        ) {
            Ok(_) => {}
            Err(error) => {
                warn!("Failed to load bank file: {:?}", file);
                warn!("Error: {:?}", error);
            }
        }
    }
}
