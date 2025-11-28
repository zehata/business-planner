use std::{collections::HashMap};

use crate::plugins::{self};

use crate::api::error::BusinessPlannerError;

pub use crate::plugins::{Plugin, PluginResponse, PluginProcess};

pub fn get_plugins() -> Result<HashMap<String, Plugin>, BusinessPlannerError> {
    Ok(plugins::get_plugins()?)
}

pub fn run_plugin(plugin_name: &str) -> Result<PluginProcess, BusinessPlannerError> {
    plugins::run_plugin(plugin_name)
}