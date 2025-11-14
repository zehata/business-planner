use std::io::Error as IoError;
use pyo3::PyErr;
use std::ffi::NulError;

pub enum PluginDiscoveryError {
    IoError(IoError),
    ReadDirectoryError,
}

impl From<IoError> for PluginDiscoveryError {
    fn from(value: IoError) -> Self {
        PluginDiscoveryError::IoError(value)
    }
}

pub enum PluginError {
    ReadPluginError(IoError),
    ConversionToCStringErr(NulError),
    ExternalPluginError(PyErr),
    PluginDiscoveryError(PluginDiscoveryError),
}

impl From<PyErr> for PluginError {
    fn from(value: PyErr) -> Self {
        PluginError::ExternalPluginError(value)
    }
}

impl From<IoError> for PluginError {
    fn from(value: IoError) -> Self {
        PluginError::ReadPluginError(value)
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