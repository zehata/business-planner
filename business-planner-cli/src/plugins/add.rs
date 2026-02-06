use std::{env, path::PathBuf};

use business_planner::api::{plugins::add_plugin, session::Session};
use clap::{Arg, ArgMatches, Command};
use enum_map::Enum;
use strum_macros::{Display, EnumString};

use crate::{
    Error, NonError,
    utils::{Menu, PathFilter, PlannerResult, prompt_select_path},
};

#[derive(Debug, Display, Enum, EnumString)]
pub enum AddPluginMenu {}

impl Menu for AddPluginMenu {
    fn get_command() -> Command {
        Command::new("add")
            .arg_required_else_help(true)
            .arg(Arg::new("plugin_path").value_parser(clap::value_parser!(PathBuf)))
    }

    async fn interactive(_session: &mut Session) -> PlannerResult {
        let current_dir = env::current_dir()?;
        let path = prompt_select_path(&current_dir, PathFilter::Directory).await?;
        add_plugin(&path)?;
        Ok(NonError::Continue)
    }

    async fn non_interactive(arg_matches: &ArgMatches, _session: &mut Session) -> PlannerResult {
        let path = arg_matches
            .get_one::<PathBuf>("plugin_path")
            .ok_or(Error::InvalidInput)?;
        add_plugin(path)?;
        Ok(NonError::Continue)
    }
}
