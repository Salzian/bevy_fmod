#![deny(
    clippy::wildcard_imports,
    clippy::unwrap_used,
    clippy::unwrap_in_result,
    clippy::get_unwrap,
    clippy::expect_used
)]

mod attributes_3d;
pub mod components;
mod error;
pub mod fmod_plugin;
pub mod fmod_studio;
pub mod prelude;
