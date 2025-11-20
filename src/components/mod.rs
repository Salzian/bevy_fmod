//! Components used for integrating FMOD in a Bevy application.
//!
//! The components provided in this module are essential for implementing spatial audio functionality,
//! including audio sources, listeners, and velocity. These components can be used individually or
//! grouped together using bundles for easier management.

#[doc(hidden)]
pub mod audio_listener;
#[doc(hidden)]
pub mod audio_source;
pub mod bundles;
#[doc(hidden)]
pub mod geometry;
#[doc(hidden)]
pub mod velocity;

#[doc(inline)]
pub use audio_listener::AudioListener;
#[doc(inline)]
pub use audio_source::AudioSource;
#[doc(inline)]
pub use geometry::Geometry;
#[doc(inline)]
pub use velocity::Velocity;
