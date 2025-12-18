use business_planner::api::{plugins::remove_plugin, session::Session};

use crate::{Error, NonError};

pub async fn parse_interactive_remove_plugins_subcommand(command: &str, _session: &mut Session) -> Result<NonError, Error> {
    remove_plugin(command)?;
    Ok(NonError::Continue)
}