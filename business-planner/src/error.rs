use std::fmt;

use crate::{api::session::{LoadSessionError, SaveSessionError}, plugins::error::{PluginDiscoveryError}};

#[derive(Debug)]
pub enum Error {
    PluginDiscoveryError(PluginDiscoveryError),
    SaveSessionError(SaveSessionError),
    LoadSessionError(LoadSessionError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Error::PluginDiscoveryError(error) => format!("While looking for plugins, encountered this error: {:?}", error),
            Error::SaveSessionError(error) => format!("While saving the session, encountered this error: {:?}", error),
            Error::LoadSessionError(error) => format!("While loading the session from file, encountered this error: {:?}", error),
        };
        write!(f, "{message}")
    }
}

impl From<PluginDiscoveryError> for Error {
    fn from(value: PluginDiscoveryError) -> Self {
        Error::PluginDiscoveryError(value)
    }
}

impl From<SaveSessionError> for Error {
    fn from(value: SaveSessionError) -> Self {
        Error::SaveSessionError(value)
    }
}

impl From<LoadSessionError> for Error {
    fn from(value: LoadSessionError) -> Self {
        Error::LoadSessionError(value)
    }
}