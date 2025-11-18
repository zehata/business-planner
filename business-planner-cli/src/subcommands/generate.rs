use business_planner::api::{plugins::{PluginDiscoveryError, get_plugins}, reports::generate_report, session::Session};

use crate::error;

pub fn get_commands () -> Result<Vec<String>, PluginDiscoveryError> {
    let plugins = get_plugins()?;

    Ok(plugins.keys().cloned().collect())
}

pub fn parse_interactive_command(command: &str, _session: &Session, _user_requested_exit: &mut bool) -> Result<(), error::Error> {
    let report = generate_report(command.to_string())?;
    println!("{:?}", report);
    Ok(())
}