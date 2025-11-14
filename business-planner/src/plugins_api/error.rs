use std::io::Error as IoError;
use pyo3::PyErr;
use std::ffi::NulError;

pub enum PluginError {
    ReadPluginError(IoError),
    ConversionToCStringErr(NulError),
    ExternalPluginError(PyErr),
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