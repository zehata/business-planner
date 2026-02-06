use std::str::FromStr;

use business_planner::api::{
    item::{IngredientItem, MaterialItem, StoreItem},
    session::Session,
};
use clap::{ArgMatches, Command};
use enum_map::Enum;
use inquire::Select;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};
use uuid::Uuid;

use crate::{
    Error, NonError,
    registry::{RegistryItemTypes, TakesRegistryItemId},
    utils::{Menu, PlannerResult, prompt_user_select_registry_item},
};

#[derive(Debug, Display, Enum, EnumString, EnumIter)]
pub enum DeleteRegistryItemMenu {}

impl Menu for DeleteRegistryItemMenu {
    fn get_command() -> Command {
        Command::new("delete")
            .no_binary_name(true)
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
        match command {
            RegistryItemTypes::Material => {
                let material_id =
                    prompt_user_select_registry_item::<MaterialItem>(session, "Material name")
                        .await?
                        .to_owned();
                session.delete::<MaterialItem>(&material_id);
            }
            RegistryItemTypes::Store => {
                let store_id = prompt_user_select_registry_item::<StoreItem>(session, "Store name")
                    .await?
                    .to_owned();
                session.delete::<StoreItem>(&store_id);
            }
            RegistryItemTypes::Ingredient => {
                let ingredient_id =
                    prompt_user_select_registry_item::<IngredientItem>(session, "Ingredient name")
                        .await?
                        .to_owned();
                session.delete::<IngredientItem>(&ingredient_id);
            }
        };
        Ok(NonError::Continue)
    }

    async fn non_interactive(arg_matches: &ArgMatches, session: &mut Session) -> PlannerResult {
        let item_type = arg_matches
            .get_one::<String>("item_type")
            .ok_or(Error::InvalidInput)?;
        let item_type = RegistryItemTypes::from_str(item_type)?;

        let id = arg_matches
            .get_one::<String>("id")
            .ok_or(Error::InvalidInput)?;
        let id = Uuid::parse_str(id)?;

        match item_type {
            RegistryItemTypes::Material => {
                session.delete::<MaterialItem>(&id);
            }
            RegistryItemTypes::Store => {
                session.delete::<StoreItem>(&id);
            }
            RegistryItemTypes::Ingredient => {
                session.delete::<IngredientItem>(&id);
            }
        };
        Ok(NonError::Continue)
    }
}

// pub fn get_delete_command() -> Command {
//     Command::new("delete")
//         .no_binary_name(true)
//         .takes_registry_item_id_arg()
// }

// pub fn parse_interactive_delete_subcommand<'a>(command: &'a str, session: &'a mut Session) -> Pin<Box<dyn Future<Output = Result<NonError, Error>> + 'a>> {
//     pin_action!({
//         let id = retrying_prompt_uuid()?;

//         match command {
//             "material" => {
//                 session.delete::<MaterialItem>(&id);
//                 Ok(NonError::Continue)
//             },
//             "store" => {
//                 session.delete::<StoreItem>(&id);
//                 Ok(NonError::Continue)
//             },
//             _ => Err(Error::InvalidInput),
//         }
//     })
// }

// pub fn parse_non_interactive_delete_subcommand<'a>(arg_matches: &'a ArgMatches, session: &'a mut Session) -> Pin<Box<dyn Future<Output = Result<NonError, Error>> + 'a>> {
//     pin_action!({
//         let item_type = arg_matches.get_one::<String>("item_type").ok_or(Error::InvalidInput)?;

//         let id = arg_matches.get_one::<String>("id").ok_or(Error::InvalidInput)?;
//         let id = Uuid::parse_str(id)?;

//         match &item_type[..] {
//             "material" => {
//                 session.delete::<MaterialItem>(&id);
//                 Ok(NonError::Continue)
//             },
//             "store" => {
//                 session.delete::<StoreItem>(&id);
//                 Ok(NonError::Continue)
//             },
//             _ => Err(Error::InvalidInput)
//         }
//     })
// }
