use business_planner::api::{
    plugins::{list_plugins, remove_plugin},
    session::Session,
};
use clap::{Arg, ArgMatches, Command};
use enum_map::Enum;
use inquire::Select;
use strum_macros::{Display, EnumString};

use crate::{
    Error, NonError,
    utils::{Menu, PlannerResult},
};

#[derive(Debug, Display, Enum, EnumString)]
pub enum RemovePluginMenu {}

impl Menu for RemovePluginMenu {
    fn get_command() -> Command {
        Command::new("remove")
            .arg_required_else_help(true)
            .arg(Arg::new("plugin_name"))
    }

    async fn interactive(_session: &mut Session) -> PlannerResult {
        let plugin_names = list_plugins()?;
        let selected_option = Select::new("Select", plugin_names.clone())
            .raw_prompt_skippable()?
            .ok_or(Error::UserCancelled)?;
        let plugin_name = plugin_names
            .get(selected_option.index)
            .expect("User cannot select if subcommands is empty");
        remove_plugin(plugin_name)?;
        Ok(NonError::Continue)
    }

    async fn non_interactive(arg_matches: &ArgMatches, _session: &mut Session) -> PlannerResult {
        let plugin_name = arg_matches
            .get_one::<String>("plugin_name")
            .ok_or(Error::InvalidInput)?;
        remove_plugin(plugin_name)?;
        Ok(NonError::Continue)
    }
}
