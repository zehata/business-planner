use std::collections::HashMap;

use crate::plugins;

pub use crate::plugins::{Plugin, error::PluginDiscoveryError};

pub fn get_plugins() -> Result<HashMap<String, Plugin>, PluginDiscoveryError> {
    plugins::get_plugins()
}