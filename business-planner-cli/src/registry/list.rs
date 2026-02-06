use std::str::FromStr;

use business_planner::api::{
    item::{IngredientItem, MaterialItem, StoreItem},
    session::Session,
};
use clap::{ArgMatches, Command};
use enum_map::Enum;
use inquire::Select;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumString};

use crate::{
    Error, NonError,
    registry::{RegistryItemTypes, TakesRegistryItemId, TakesRegistryItemType},
    utils::{Menu, PlannerResult, print_list},
};

#[derive(Debug, Display, Enum, EnumString)]
pub enum ListRegistryItemMenu {}

impl Menu for ListRegistryItemMenu {
    fn get_command() -> Command {
        Command::new("list")
            .no_binary_name(true)
            .takes_registry_item_type_arg()
            .takes_registry_item_id_arg()
    }

    async fn interactive(session: &mut Session) -> PlannerResult {
        let commands = RegistryItemTypes::iter().collect::<Vec<_>>();
        let command_names = commands
            .iter()
            .map(|command| format!("{}", command))
            .collect();
        let selected_command = Select::new("Select", command_names)
            .raw_prompt_skippable()?
            .ok_or(Error::UserCancelled)?;
        let command = commands
            .get(selected_command.index)
            .expect("User cannot select if subcommands is empty");

        let items = match command {
            RegistryItemTypes::Material => session.list::<MaterialItem>().collect::<Vec<_>>(),
            RegistryItemTypes::Store => session.list::<StoreItem>().collect::<Vec<_>>(),
            RegistryItemTypes::Ingredient => session.list::<IngredientItem>().collect::<Vec<_>>(),
        }.iter().map(|(_, item_name)| {
            item_name.unwrap_or("")
        }).collect::<Vec<_>>();
        print_list(items);
        Ok(NonError::Continue)
    }

    async fn non_interactive(arg_matches: &ArgMatches, session: &mut Session) -> PlannerResult {
        let item_type = arg_matches
            .get_one::<String>("item_type")
            .ok_or(Error::InvalidInput)?;
        let item_type = RegistryItemTypes::from_str(item_type)?;

        let items = match item_type {
            RegistryItemTypes::Material => session.list::<MaterialItem>().collect::<Vec<_>>(),
            RegistryItemTypes::Store => session.list::<StoreItem>().collect::<Vec<_>>(),
            RegistryItemTypes::Ingredient => session.list::<IngredientItem>().collect::<Vec<_>>(),
        }.iter().map(|(_, item_name)| {
            item_name.unwrap_or("")
        }).collect::<Vec<_>>();
        print_list(items);
        Ok(NonError::Continue)
    }
}

// pub fn get_list_subcommand() -> Command {
//     Command::new("create")
//         .no_binary_name(true)
//         .takes_registry_item_type_arg()
// }

// pub async fn parse_interactive_list_subcommand(command: &str, session: &mut Session) -> Result<NonError, Error> {
//     match command {
//         "material" => {
//             session.list::<MaterialItem>();
//             Ok(NonError::Continue)
//         },
//         "store" => {
//             session.list::<StoreItem>();
//             Ok(NonError::Continue)
//         },
//         _ => Err(Error::InvalidInput),
//     }
// }

// pub async fn parse_non_interactive_list_subcommand(arg_matches: &ArgMatches, session: &mut Session) -> Result<NonError, Error> {
//     match arg_matches.subcommand() {
//         Some(("material", _)) => {
//             session.list::<MaterialItem>();
//             Ok(NonError::Continue)
//         },
//         Some(("store", _)) => {
//             session.list::<StoreItem>();
//             Ok(NonError::Continue)
//         },
//         _ => Err(Error::InvalidInput),
//     }
// }
