use std::path::PathBuf;

use bevy_mod_sysfail::{FailureMode, LogLevel};

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Fmod(libfmod::Error),
    PathConversion(PathBuf),
    IO(std::io::Error),
}

impl FailureMode for Error {
    fn log_level(&self) -> LogLevel {
        LogLevel::Error
    }

    // TODO implement this
    type ID = ();

    fn identify(&self) -> Self::ID {}

    fn display(&self) -> Option<String> {
        match self {
            Error::PathConversion(path) => Some(format!("Failed to convert path: {:?}", path)),
            Error::Fmod(fmod_error) => Some(format!("FMOD error: {:?}", fmod_error)),
            Error::IO(io_error) => Some(format!("IO error: {:?}", io_error)),
        }
    }
}

impl From<libfmod::Error> for Error {
    fn from(value: libfmod::Error) -> Self {
        Self::Fmod(value)
    }
}
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}
