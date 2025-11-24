use business_planner::api::{plugins::{get_plugins, run_plugin}, session::Session};

use crate::{Error, NonError};

pub fn get_run_plugins_interactive_subcommand () -> Result<Vec<String>, Error> {
    Ok(get_plugins()?.keys().cloned().collect())
}

pub async fn parse_interactive_run_plugins_subcommand(command: &str, _session: &mut Session) -> Result<NonError, Error> {
    println!("{}", run_plugin(command)?);
    Ok(NonError::Continue)
}