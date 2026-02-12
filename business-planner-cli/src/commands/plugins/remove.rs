use business_planner::api::{
    plugins::{list_plugins, remove_plugin},
    session::Session,
};
use clap::{Arg, ArgMatches, Command};
use inquire::Select;

use crate::{
    Error, NonError,
    utils::{Menu, NoSubcommands, PlannerResult},
};

pub struct RemovePluginMenu {}

impl Menu for RemovePluginMenu {
    type SubcommandsEnum = NoSubcommands;

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
