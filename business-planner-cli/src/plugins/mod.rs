use business_planner::api::session::Session;
use clap::Command;

use crate::{Error, NonError, plugins::run::{get_run_plugins_interactive_subcommand, parse_interactive_run_plugins_subcommand}, shells::interactive};

pub mod run;

pub fn get_plugins_subcommand () -> Command {
    Command::new("plugins")
        .subcommands([
            Command::new("add"),
            Command::new("remove"),
            Command::new("run"),
        ])
}

pub async fn parse_interactive_plugins_subcommand(command: &str, session: &mut Session) -> Result<NonError, Error> {
    match command {
        "run" => {
            interactive::shell(
                get_run_plugins_interactive_subcommand()?,
                parse_interactive_run_plugins_subcommand,
                session
            ).await
        },
        _ => unimplemented!()
    }
}