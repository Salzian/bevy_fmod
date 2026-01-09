use std::fs::canonicalize;
use std::path::Path;

use bevy::log::error;
use bevy::prelude::{Deref, DerefMut, Resource, debug};
#[cfg(feature = "live-update")]
use libfmod::ffi::FMOD_STUDIO_INIT_LIVEUPDATE;
use libfmod::ffi::{
    FMOD_INIT_3D_RIGHTHANDED, FMOD_STUDIO_INIT_NORMAL, FMOD_STUDIO_LOAD_BANK_NORMAL,
};
use libfmod::{Studio, System};

/// A resource that wraps the `Studio` object from the FMOD library.
///
/// This struct provides convenient access to the FMOD Studio API within the Bevy ECS.
/// It derives `Deref` and `DerefMut` to allow direct access to the inner `Studio` object.
#[derive(Resource, Deref, DerefMut)]
pub struct FmodStudio(pub Studio);

impl FmodStudio {
    pub(crate) fn new(
        banks_paths: &[&'static str],
        plugin_paths: Option<&[&'static str]>,
    ) -> crate::Result<Self> {
        let studio = Self::init_studio()?;
        let studio_core = studio.get_core_system()?;

        if let Some(plugin_paths) = plugin_paths {
            plugin_paths.iter().for_each(|plugin_path| {
                match canonicalize(Path::new(plugin_path)) {
                    Ok(path) => Self::load_plugin(studio_core, path.as_path()),
                    Err(e) => error!("Failed to canonicalize {plugin_path}: {e}"),
                }
            });
        }

        banks_paths
            .iter()
            .for_each(|bank_path| match canonicalize(Path::new(bank_path)) {
                Ok(path) => Self::load_bank(&studio, path.as_path()),
                Err(e) => error!("Failed to canonicalize {bank_path}: {e}"),
            });

        Ok(FmodStudio(studio))
    }

    fn load_plugin(studio_core: System, plugin_path: &Path) {
        debug!("Loading FMOD plugin from: {plugin_path:?}");

        let filename = plugin_path.to_str().expect("Path should be valid unicode");
        let priority = None;

        if let Err(e) = studio_core.load_plugin(filename, priority) {
            error!("Could not load FMOD plugin: {e}")
        };
    }

    fn load_bank(studio: &Studio, bank_path: &Path) {
        debug!("Loading audio banks from: {:?}", bank_path);

        let filename = bank_path.to_str().expect("Path should be valid unicode");
        let flags = FMOD_STUDIO_LOAD_BANK_NORMAL;

        if let Err(e) = studio.load_bank_file(filename, flags) {
            error!("Could not load bank: {e}")
        };
    }

    fn init_studio() -> crate::Result<Studio> {
        let studio = Studio::create()?;

        let studio_flags = FMOD_STUDIO_INIT_NORMAL;

        #[cfg(feature = "live-update")]
        let studio_flags = studio_flags | FMOD_STUDIO_INIT_LIVEUPDATE;

        debug!("Initializing FMOD studio with flags: {}", studio_flags);

        studio.initialize(1024, studio_flags, FMOD_INIT_3D_RIGHTHANDED, None)?;

        Ok(studio)
    }
}
