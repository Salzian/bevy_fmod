//! Re-exports the most commonly used types, traits, and functions in this crate.
//!
//! ## Usage
//!
//! ```
//! use bevy_fmod::prelude::*;
//! ```

pub use crate::components::audio_listener::AudioListener;
pub use crate::components::audio_source::AudioSource;
pub use crate::components::bundles::SpatialAudioBundle;
pub use crate::components::bundles::SpatialListenerBundle;
pub use crate::components::velocity::Velocity;
pub use crate::fmod_plugin::FmodPlugin;
pub use crate::fmod_studio::FmodStudio;
