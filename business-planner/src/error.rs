use std::fmt;
use std::string::FromUtf8Error;

use crate::{api::session::{LoadSessionError, SaveSessionError}, plugins::error::{PluginDiscoveryError, PluginError}};

#[derive(Debug)]
pub enum Error {
    PluginDiscoveryError(PluginDiscoveryError),
    PluginError(PluginError),
    FromUtf8Error(FromUtf8Error),
    SaveSessionError(SaveSessionError),
    LoadSessionError(LoadSessionError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Error::PluginDiscoveryError(error) => format!("While looking for plugins, encountered this error: {:?}", error),
            Error::PluginError(error) => format!("While running plugin, encountered this error: {:?}", error),
            Error::SaveSessionError(error) => format!("While saving the session, encountered this error: {:?}", error),
            Error::LoadSessionError(error) => format!("While loading the session from file, encountered this error: {:?}", error),
            _ => dbg!(self).to_string()
        };
        write!(f, "{message}")
    }
}

impl From<PluginDiscoveryError> for Error {
    fn from(value: PluginDiscoveryError) -> Self {
        Error::PluginDiscoveryError(value)
    }
}

impl From<PluginError> for Error {
    fn from(value: PluginError) -> Self {
        Error::PluginError(value)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(value: FromUtf8Error) -> Self {
        Error::FromUtf8Error(value)
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