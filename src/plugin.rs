use crate::PlaySoundEvent;
use bevy::app::{App, Plugin};
use bevy::audio::AudioSinkPlayback;
use bevy::log::{debug, trace};
use bevy::math::Vec3;
use bevy::prelude::{
    Added, Commands, Component, Entity, EventReader, GlobalTransform, NonSend, PostUpdate, Query,
    Res, Resource, Startup, Update, World,
};
use bevy::time::Time;
use bevy_mod_sysfail::sysfail;
use libfmod::ffi::{FMOD_INIT_NORMAL, FMOD_STUDIO_INIT_NORMAL, FMOD_STUDIO_LOAD_BANK_NORMAL};
use libfmod::{Attributes3d, EventDescription, Studio, Vector};
use std::env::var;
use std::fs::{canonicalize, read_dir};
use std::path::{Path, PathBuf};

#[derive(Component)]
pub struct AudioListener {
    previous_position: Vec3,
}

impl Default for AudioListener {
    fn default() -> Self {
        AudioListener {
            previous_position: Vec3::default(),
        }
    }
}

#[derive(Component)]
pub struct AudioSource {
    pub name: &'static str,
}

// This should be OK I think since we should only have one event instance per entity
unsafe impl Sync for UnsafeEventInstance {}
unsafe impl Send for UnsafeEventInstance {}

struct UnsafeEventInstance {
    pointer: libfmod::EventInstance,
}

#[derive(Component)]
pub struct AudioSourcePlayer {
    pub name: &'static str,
    fmod_event: UnsafeEventInstance,
    previous_position: Vec3,
}

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
            .add_systems(
                Update,
                (
                    Self::play_incoming_events,
                    Self::check_for_new_sources,
                    Self::update_sources,
                ),
            )
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

    fn update_sources(
        mut query: Query<(&GlobalTransform, &mut AudioSourcePlayer)>,
        time: Res<Time>,
    ) {
        for (transform, mut source) in query.iter_mut() {
            let pos = transform.translation();
            let fwd = transform.forward();
            let up = transform.up();

            let delta = pos - source.previous_position;
            let vel = delta / time.delta_seconds();
            source.previous_position = pos;

            let source_attributes = attributes3d(pos, vel, fwd, up);

            source
                .fmod_event
                .pointer
                .set_3d_attributes(source_attributes)
                .unwrap();
        }
    }

    #[sysfail(log(level = "error"))]
    fn update(
        studio: NonSend<Studio>,
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

            let listener_attributes = attributes3d(pos, vel, fwd, up);

            studio.set_listener_attributes(0, listener_attributes, None)?;
        }

        studio.update()?;
        Ok(())
    }

    fn check_for_new_sources(
        mut commands: Commands,
        query: Query<(Entity, &AudioSource), Added<AudioSource>>,
        studio: NonSend<Studio>,
    ) {
        for (ent, source) in query.iter() {
            let event_description = studio.get_event(source.name).unwrap();
            let instance = event_description.create_instance().unwrap();

            // Start the effect already
            instance.start().unwrap();

            commands.entity(ent).insert(AudioSourcePlayer {
                name: source.name,
                fmod_event: UnsafeEventInstance { pointer: instance },
                previous_position: Vec3::ZERO,
            });
        }
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

fn attributes3d(pos: Vec3, vel: Vec3, fwd: Vec3, up: Vec3) -> Attributes3d {
    Attributes3d {
        position: Vector {
            x: pos.x,
            y: pos.y,
            z: -pos.z,
        },
        velocity: Vector {
            x: vel.x,
            y: vel.y,
            z: -vel.z,
        },
        forward: Vector {
            x: fwd.x,
            y: fwd.y,
            z: -fwd.z,
        },
        up: Vector {
            x: up.x,
            y: up.y,
            z: -up.z,
        },
    }
}

impl AudioSinkPlayback for AudioSourcePlayer {
    fn volume(&self) -> f32 {
        let (volume, _final_volume) = self.fmod_event.pointer.get_volume().unwrap();
        volume
    }

    fn set_volume(&self, volume: f32) {
        self.fmod_event.pointer.set_volume(volume).unwrap();
    }

    fn speed(&self) -> f32 {
        todo!()
    }

    fn set_speed(&self, speed: f32) {
        todo!()
    }

    fn play(&self) {
        // AudioSinkPlayback does not have a resume function so we go for this
        if self.is_paused() {
            self.fmod_event.pointer.set_paused(false).unwrap();
        } else {
            self.fmod_event.pointer.start().unwrap();
        }
    }

    fn pause(&self) {
        self.fmod_event.pointer.set_paused(true).unwrap();
    }

    fn is_paused(&self) -> bool {
        self.fmod_event.pointer.get_paused().unwrap()
    }

    fn stop(&self) {
        // Todo: configurable StopMode
        self.fmod_event
            .pointer
            .stop(libfmod::StopMode::AllowFadeout)
            .unwrap();
    }

    fn empty(&self) -> bool {
        !self.fmod_event.pointer.is_valid()
    }
}

impl Drop for AudioSourcePlayer {
    fn drop(&mut self) {
        self.fmod_event
            .pointer
            .release()
            .expect("Error releasing FMOD event instance");
    }
}
