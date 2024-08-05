use std::fs::canonicalize;
use std::path::{Path, PathBuf};

use bevy::log::trace;
use bevy::prelude::Resource;
#[cfg(feature = "live-update")]
use libfmod::ffi::FMOD_STUDIO_INIT_LIVEUPDATE;
use libfmod::ffi::{
    FMOD_INIT_3D_RIGHTHANDED, FMOD_STUDIO_INIT_NORMAL, FMOD_STUDIO_LOAD_BANK_NORMAL,
};
use libfmod::{Bank, Studio, System};

use crate::error;
use crate::error::Error;

#[derive(Resource)]
pub struct FmodStudio(pub Studio);

impl FmodStudio {
    pub(crate) fn new(
        banks_paths: &[&'static str],
        plugin_paths: Option<&[&'static str]>,
    ) -> error::Result<FmodStudio> {
        let studio = Self::init_studio()?;
        let studio_core = studio.get_core_system()?;

        if let Some(plugin_paths) = plugin_paths {
            for plugin_path in plugin_paths.iter() {
                let path = canonicalize(Path::new(plugin_path))?;

                trace!("Loading FMOD plugins from: {:?}", path);
                Self::load_plugin(studio_core, path)?;
            }
        }

        for bank_path in banks_paths.iter() {
            let path = canonicalize(Path::new(bank_path))?;

            trace!("Loading audio banks from: {:?}", path);
            Self::load_bank(&studio, path.as_path())?;
        }

        Ok(FmodStudio(studio))
    }

    fn load_plugin<'path, 'r>(studio_core: System, plugin_path: PathBuf) -> error::Result<()>
    where
        'path: 'r,
    {
        let plugin_path_str = plugin_path
            .to_str()
            .ok_or(Error::PathConversion(plugin_path.to_owned()))?;

        studio_core
            .load_plugin(plugin_path_str, None)
            .map(|_| ())
            .map_err(Error::Fmod)
    }

    fn load_bank(studio: &Studio, bank_path: &Path) -> error::Result<Bank> {
        let bank_path = bank_path
            .to_str()
            .ok_or(Error::PathConversion(bank_path.to_owned()))?;

        studio
            .load_bank_file(bank_path, FMOD_STUDIO_LOAD_BANK_NORMAL)
            .map_err(Error::Fmod)
    }

    fn init_studio() -> Result<Studio, libfmod::Error> {
        let studio = Studio::create()?;

        let studio_flags = FMOD_STUDIO_INIT_NORMAL;

        #[cfg(feature = "live-update")]
        let studio_flags = studio_flags | FMOD_STUDIO_INIT_LIVEUPDATE;

        trace!("Initializing FMOD studio with flags: {}", studio_flags);

        studio.initialize(1024, studio_flags, FMOD_INIT_3D_RIGHTHANDED, None)?;
        Ok(studio)
    }
}
