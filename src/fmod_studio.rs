use std::env::var;
use std::fs::canonicalize;
use std::path::{Path, PathBuf};

use crate::attributes_3d::attributes3d;
use bevy::prelude::{debug, trace, Resource, Transform, Vec3};
#[cfg(feature = "live-update")]
use libfmod::ffi::FMOD_STUDIO_INIT_LIVEUPDATE;
use libfmod::ffi::{
    FMOD_INIT_3D_RIGHTHANDED, FMOD_STUDIO_INIT_NORMAL, FMOD_STUDIO_LOAD_BANK_NORMAL,
};
use libfmod::Studio;

#[derive(Resource)]
pub struct FmodStudio(pub Studio);

impl FmodStudio {
    pub(crate) fn new(banks_paths: &[&'static str]) -> Self {
        let studio = Self::init_studio();
        let project_root = var("CARGO_MANIFEST_DIR").unwrap();

        banks_paths.iter().for_each(|bank_path| {
            let mut path = canonicalize(Path::new(bank_path))
                .expect("Failed to canonicalize provided audio banks directory path.");

            trace!("Canonicalized audio banks directory path: {:?}", path);

            if path.is_relative() {
                let relative_path = path;
                path = PathBuf::new();
                path.push(project_root.clone());
                path.push(relative_path)
            }

            debug!("Loading audio banks from: {:?}", path);

            Self::load_bank(&studio, path.as_path());
        });

        FmodStudio(studio)
    }

    fn load_bank(studio: &Studio, bank_path: &Path) {
        studio
            .load_bank_file(
                bank_path
                    .to_str()
                    .expect("Failed to convert path to string"),
                FMOD_STUDIO_LOAD_BANK_NORMAL,
            )
            .expect("Could not load bank.");
    }

    fn init_studio() -> Studio {
        let studio = Studio::create().expect("Failed to create FMOD studio");

        let studio_flags = FMOD_STUDIO_INIT_NORMAL;

        #[cfg(feature = "live-update")]
        let studio_flags = studio_flags | FMOD_STUDIO_INIT_LIVEUPDATE;

        debug!("Initializing FMOD studio with flags: {}", studio_flags);

        studio
            .initialize(1024, studio_flags, FMOD_INIT_3D_RIGHTHANDED, None)
            .expect("Failed to initialize FMOD studio");

        studio
    }

    pub fn play_event_at(&self, transform: Transform, path_or_id: &str) {
        let event_description = self.0.get_event(path_or_id).unwrap();
        let event_instance = event_description.create_instance().unwrap();

        event_instance
            .set_3d_attributes(attributes3d(
                transform.translation,
                Vec3::ZERO,
                transform.forward().into(),
                transform.up().into(),
            ))
            .unwrap();

        event_instance.start().unwrap();
        event_instance.release().unwrap();
    }
}
