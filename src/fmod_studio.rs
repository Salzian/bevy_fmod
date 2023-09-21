use std::env::var;
use std::fs::{canonicalize, read_dir};
use std::path::{Path, PathBuf};

use bevy::ecs::system::Resource;
use bevy::log::{debug, trace};
use libfmod::ffi::{
    FMOD_INIT_3D_RIGHTHANDED, FMOD_STUDIO_INIT_NORMAL, FMOD_STUDIO_LOAD_BANK_NORMAL,
};
use libfmod::Studio;

#[derive(Resource)]
pub(crate) struct FmodStudio(pub(crate) Studio);

impl FmodStudio {
    pub(crate) fn new(banks_path: &str) -> Self {
        let banks_path = Path::new(banks_path);
        let studio = init_studio();

        let mut path = canonicalize(banks_path)
            .expect("Failed to canonicalize provided audio banks directory path.");

        trace!("Canonicalized audio banks directory path: {:?}", path);

        if path.is_relative() {
            let project_root = PathBuf::from(var("CARGO_MANIFEST_DIR").unwrap());
            path = PathBuf::new();
            path.push(project_root);
            path.push(banks_path);
        }

        debug!("Loading audio banks from: {:?}", path);

        Self::load_banks(&studio, path.as_path()).expect("Failed to load audio banks.");

        FmodStudio(studio)
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

fn init_studio() -> Studio {
    let studio = Studio::create().expect("Failed to create FMOD studio");

    studio
        .initialize(
            1024,
            FMOD_STUDIO_INIT_NORMAL,
            FMOD_INIT_3D_RIGHTHANDED,
            None,
        )
        .expect("Failed to initialize FMOD studio");

    studio
}
