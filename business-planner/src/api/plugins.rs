use std::collections::HashMap;

use crate::plugins;

use crate::api::error::BusinessPlannerError;

pub use crate::plugins::{Plugin};

pub fn get_plugins() -> Result<HashMap<String, Plugin>, BusinessPlannerError> {
    Ok(plugins::get_plugins()?)
}