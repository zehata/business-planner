use std::ffi::NulError;
use std::io::Error as IoError;
use trash::Error as TrashError;

#[derive(Debug)]
pub enum PluginManagementError {
    IoError(IoError),
    TrashError(TrashError),
    ReadDirectoryError,
    PluginNotFound,
}

impl From<IoError> for PluginManagementError {
    fn from(value: IoError) -> Self {
        PluginManagementError::IoError(value)
    }
}

impl From<TrashError> for PluginManagementError {
    fn from(value: TrashError) -> Self {
        PluginManagementError::TrashError(value)
    }
}

#[derive(Debug)]
pub enum PluginError {
    IoError(IoError),
    ConversionToCStringErr(NulError),
    PluginManagementError(PluginManagementError),
    PluginMissingError,
    IncompleteDataSource,
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

impl From<PluginManagementError> for PluginError {
    fn from(value: PluginManagementError) -> Self {
        PluginError::PluginManagementError(value)
    }
}
