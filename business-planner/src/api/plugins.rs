use std::path::PathBuf;

use crate::plugins::{self, PluginProcess};

use crate::api::error::BusinessPlannerError;

pub use crate::plugins::{DataRequest, DataResponse, Plugin, PluginResponse};

pub fn list_plugins() -> Result<Vec<String>, BusinessPlannerError> {
    Ok(plugins::list_plugins()?)
}

pub fn run_plugin(plugin_name: &str) -> Result<PluginProcess, BusinessPlannerError> {
    plugins::run_plugin(plugin_name)
}

pub fn remove_plugin(plugin_name: &str) -> Result<(), BusinessPlannerError> {
    plugins::remove_plugin(plugin_name)
}

pub fn add_plugin(path: &PathBuf) -> Result<(), BusinessPlannerError> {
    Ok(plugins::add_plugin(path)?)
}
