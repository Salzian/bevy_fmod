//! # Utilities
//!
//! Collection of useful plugins, components or systems that are not part of the FMOD API but help
//! when developing bevy games with FMOD.

mod mute_when_unfocused;

#[doc(inline)]
pub use mute_when_unfocused::MuteWhenUnfocusedPlugin;
