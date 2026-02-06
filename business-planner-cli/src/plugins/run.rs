use business_planner::api::{
    item::StoreItem,
    plugins::{self, DataRequest, PluginResponse, list_plugins},
    session::Session,
};
use clap::{Arg, ArgMatches, Command};
use enum_map::Enum;
use inquire::Select;
use strum_macros::{Display, EnumString};

use crate::{
    Error, NonError,
    registry::get_registry_item_types,
    utils::{Menu, PlannerResult, prompt_user_select_registry_item},
};

#[derive(Debug, Display, Enum, EnumString)]
pub enum RunPluginMenu {}

impl Menu for RunPluginMenu {
    fn get_command() -> clap::Command {
        Command::new("run")
            .arg_required_else_help(true)
            .arg(Arg::new("plugin_name"))
    }

    async fn interactive(session: &mut Session) -> PlannerResult {
        let plugin_names = list_plugins()?;
        let selected_option = Select::new("Select", plugin_names.clone())
            .raw_prompt_skippable()?
            .ok_or(Error::UserCancelled)?;
        let plugin_name = plugin_names
            .get(selected_option.index)
            .expect("User cannot select if subcommands is empty");
        run_plugin(plugin_name, session).await
    }

    async fn non_interactive(arg_matches: &ArgMatches, session: &mut Session) -> PlannerResult {
        let plugin_name = arg_matches
            .get_one::<String>("plugin_name")
            .ok_or(Error::InvalidInput)?;
        run_plugin(plugin_name, session).await
    }
}

async fn run_plugin(plugin_name: &str, session: &mut Session) -> PlannerResult {
    let mut plugin_process = plugins::run_plugin(plugin_name)?;

    while let Some(response) = plugin_process.responses.try_next() {
        match response {
            Ok(response) => match response {
                PluginResponse::AnyDataRequest(mut data_request) => {
                    let item_type =
                        Select::new("NodeItem type", get_registry_item_types()).prompt()?;
                    match &item_type[..] {
                        "store" => {
                            let id =
                                *prompt_user_select_registry_item::<StoreItem>(session, "Store id")
                                    .await?;
                            let data = session.resolve::<StoreItem>(&id)?;
                            data_request.send_response(data.clone())?;
                        }
                        _ => return Err(Error::InvalidInput),
                    };
                }
                PluginResponse::Message(message) => {
                    println!("{message}");
                }
                PluginResponse::Report(message) => {
                    println!("{message}");
                }
                PluginResponse::ProcessEnded => return Ok(NonError::Continue),
            },
            Err(error) => return Err(Error::BusinessPlannerError(error)),
        }
    }

    plugin_process.await_exit()?;
    Ok(NonError::Continue)
}
