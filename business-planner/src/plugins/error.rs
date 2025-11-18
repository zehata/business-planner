use std::io::Error as IoError;
use std::ffi::NulError;

#[derive(Debug)]
pub enum PluginDiscoveryError {
    IoError(IoError),
    ReadDirectoryError,
    PluginNotFound,
}

impl From<IoError> for PluginDiscoveryError {
    fn from(value: IoError) -> Self {
        PluginDiscoveryError::IoError(value)
    }
}

#[derive(Debug)]
pub enum PluginError {
    IoError(IoError),
    ConversionToCStringErr(NulError),
    PluginDiscoveryError(PluginDiscoveryError),
    PluginMissingError,
}

impl From<IoError> for PluginError {
    fn from(value: IoError) -> Self {
        PluginError::IoError(value)
    }
}

impl From<NulError> for PluginError {
    fn from(value: NulError) -> Self {
        PluginError::ConversionToCStringErr(value)
    }
}

impl From<PluginDiscoveryError> for PluginError {
    fn from(value: PluginDiscoveryError) -> Self {
        PluginError::PluginDiscoveryError(value)
    }
}