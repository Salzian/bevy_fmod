use bevy::app::{App, Plugin};
use bevy::log::{debug, trace};
use bevy::prelude::{EventReader, NonSend, PostUpdate, Resource, Startup, Update, World};
use bevy_mod_sysfail::sysfail;
use libfmod::ffi::{FMOD_INIT_NORMAL, FMOD_STUDIO_INIT_NORMAL, FMOD_STUDIO_LOAD_BANK_NORMAL};
use libfmod::{EventDescription, Studio};
use play_sound_event::PlaySoundEvent;
use std::env::var;
use std::fs::{canonicalize, read_dir};
use std::path::{Path, PathBuf};

pub struct FmodPlugin {
    pub audio_banks_directory: &'static str,
}

#[derive(Resource)]
struct Config {
    audio_banks_directory: &'static str,
}

impl Plugin for FmodPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlaySoundEvent>()
            .insert_resource(Config {
                audio_banks_directory: self.audio_banks_directory,
            })
            .add_systems(Startup, Self::startup)
            .add_systems(Update, Self::play_incoming_events)
            .add_systems(PostUpdate, Self::update);
    }
}

impl FmodPlugin {
    fn startup(world: &mut World) {
        let studio = Self::init_studio();
        let config = world.get_resource::<Config>().unwrap();

        let mut path = canonicalize(config.audio_banks_directory)
            .expect("Failed to canonicalize provided audio banks directory path.");

        trace!("Canonicalized audio banks directory path: {:?}", path);

        if path.is_relative() {
            let project_root = PathBuf::from(var("CARGO_MANIFEST_DIR").unwrap());
            path = PathBuf::new();
            path.push(project_root);
            path.push(config.audio_banks_directory);
        }

        debug!("Loading audio banks from: {:?}", path);

        Self::load_banks(&studio, path.as_path()).expect("Failed to load audio banks.");
        world.insert_non_send_resource(studio);
    }

    #[sysfail(log(level = "error"))]
    fn update(studio: NonSend<Studio>) -> anyhow::Result<()> {
        studio.update()?;
        Ok(())
    }

    #[sysfail(log(level = "error"))]
    fn play_incoming_events(
        mut events: EventReader<PlaySoundEvent>,
        studio: NonSend<Studio>,
    ) -> anyhow::Result<()> {
        for event in events.iter() {
            let event_description = studio.get_event(event.0)?;
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

    fn init_studio() -> Studio {
        let studio = Studio::create().expect("Failed to create FMOD studio");
        studio
            .initialize(1024, FMOD_STUDIO_INIT_NORMAL, FMOD_INIT_NORMAL, None)
            .expect("Failed to initialize FMOD studio");
        studio
    }

    fn load_banks(studio: &Studio, banks_dir: &Path) -> anyhow::Result<()> {
        // Collect all files within the fmod/Example/Build/Desktop directory
        let files = read_dir(banks_dir)
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
        files.iter().for_each(|file| trace!("{:?}", file));

        // Load all files as banks
        for file in files {
            debug!("Loading bank file: {:?}", file);

            studio.load_bank_file(
                file.to_str().expect("Failed to convert path to string"),
                FMOD_STUDIO_LOAD_BANK_NORMAL,
            )?;
        }

        Ok(())
    }
}
