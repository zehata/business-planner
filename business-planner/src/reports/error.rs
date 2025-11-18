use std::{fmt, string::FromUtf8Error};

use crate::{plugins::error::{PluginDiscoveryError, PluginError}};

#[derive(Debug)]
pub enum ReportGenerationError {
    PluginDiscoveryError(PluginDiscoveryError),
    PluginError(PluginError),
    PluginNotFound(PluginNotFound),
    OutputConversionError(FromUtf8Error),
}

pub struct PluginNotFound {
    plugin_name: String
}

impl PluginNotFound {
    pub fn new(plugin_name: String) -> PluginNotFound {
        PluginNotFound { plugin_name }
    }
}

impl fmt::Debug for PluginNotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The plugin \"{}\" was not found, could it have been renamed, moved or deleted?", self.plugin_name)
    }
}

impl From<PluginError> for ReportGenerationError {
    fn from(value: PluginError) -> Self {
        ReportGenerationError::PluginError(value)
    }
}

impl From<PluginDiscoveryError> for ReportGenerationError {
    fn from(value: PluginDiscoveryError) -> Self {
        ReportGenerationError::PluginDiscoveryError(value)
    }
}

impl From<FromUtf8Error> for ReportGenerationError {
    fn from(value: FromUtf8Error) -> Self {
        ReportGenerationError::OutputConversionError(value)
    }
}