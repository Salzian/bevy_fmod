//! Error types for bevy_fmod

use thiserror::Error;

/// Error types
#[derive(Error, Debug)]
pub enum Error {
    /// Errors coming from libfmod
    #[error(transparent)]
    LibfmodError(#[from] libfmod::Error),
    /// IO errors
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
