#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(clippy::unwrap_used, clippy::wildcard_imports)]

mod attributes_3d;
pub mod components;
pub mod error;
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

// Re-export libfmod for plugin authors and error handling:
pub use libfmod;

pub(crate) type Result<T> = std::result::Result<T, error::Error>;
