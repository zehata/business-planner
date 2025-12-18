use std::env;

use business_planner::api::{plugins::{add_plugin, list_plugins}, session::Session};
use clap::Command;

use crate::{Error, NonError, plugins::{remove::parse_interactive_remove_plugins_subcommand, run::parse_interactive_run_plugins_subcommand}, shells::interactive, utils::prompt_select_path::{PathFilter, prompt_select_path}};

pub mod run;
pub mod remove;

pub fn get_plugins_subcommand () -> Command {
    Command::new("plugins")
        .subcommands([
            Command::new("run"),
            Command::new("add"),
            Command::new("remove"),
        ])
}

pub fn get_plugin_names() -> Result<Vec<String>, Error> {
    Ok(list_plugins()?)
}

pub async fn parse_interactive_plugins_subcommand(command: &str, session: &mut Session) -> Result<NonError, Error> {
    match command {
        "run" => {
            interactive::shell(
                get_plugin_names()?,
                parse_interactive_run_plugins_subcommand,
                session
            ).await
        },
        "add" => {
            let current_dir = env::current_dir()?;
            let path = prompt_select_path(&current_dir, PathFilter::Directory).await?;
            println!("{:?}", path);
            add_plugin(&path)?;
            Ok(NonError::Continue)
        },
        "remove" => {
            interactive::shell(
                get_plugin_names()?,
                parse_interactive_remove_plugins_subcommand,
                session
            ).await
        },
        _ => Err(Error::InvalidInput)
    }
}