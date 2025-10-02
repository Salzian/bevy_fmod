#![doc = include_str!("../README.md")]
#![deny(clippy::wildcard_imports, missing_docs)]

mod attributes_3d;
pub mod components;
#[doc(hidden)]
pub mod fmod_plugin;
#[doc(hidden)]
pub mod fmod_studio;
pub mod prelude;
#[cfg(feature = "utilities")]
pub mod utilities;

#[doc(inline)]
pub use fmod_plugin::FmodPlugin;
#[doc(inline)]
pub use fmod_studio::FmodStudio;

// Re-export libfmod for plugin authors:
pub use libfmod;
