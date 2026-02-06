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
    registry::{RegistryItemTypes, TakesRegistryItemId, TakesRegistryItemType},
    utils::{Menu, PlannerResult, prompt_user_select_registry_item},
};

#[derive(Debug, Display, Enum, EnumString, EnumIter)]
pub enum ReadRegistryItemMenu {}

impl Menu for ReadRegistryItemMenu {
    fn get_command() -> Command {
        Command::new("read")
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
        match command {
            RegistryItemTypes::Material => {
                let material_id =
                    prompt_user_select_registry_item::<MaterialItem>(session, "Material name")
                        .await?
                        .to_owned();
                println!("{}", session.read::<MaterialItem>(&material_id).expect(""));
            }
            RegistryItemTypes::Store => {
                let store_id = prompt_user_select_registry_item::<StoreItem>(session, "Store name")
                    .await?
                    .to_owned();
                println!("{}", session.read::<StoreItem>(&store_id).expect(""));
            }
            RegistryItemTypes::Ingredient => {
                let ingredient_id =
                    prompt_user_select_registry_item::<IngredientItem>(session, "Ingredient name")
                        .await?
                        .to_owned();
                println!(
                    "{}",
                    session.read::<IngredientItem>(&ingredient_id).expect("")
                );
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
                let material = session
                    .read::<MaterialItem>(&id)
                    .ok_or(Error::InvalidInput)?;
                println!("{}", material);
            }
            RegistryItemTypes::Store => {
                let store = session.read::<StoreItem>(&id).ok_or(Error::InvalidInput)?;
                println!("{}", store);
            }
            RegistryItemTypes::Ingredient => {
                let store = session
                    .read::<IngredientItem>(&id)
                    .ok_or(Error::InvalidInput)?;
                println!("{}", store);
            }
        };
        Ok(NonError::Continue)
    }
}

// async fn read_item_interactive<'a>(command: &'a str, session: &'a mut Session) -> PlannerResult {
//     match command {
//         "material" => {
//             let material = prompt_user_select_registry_item::<MaterialItem>(session, "Material id").await?;
//             println!("{}", material);
//             Ok(NonError::Continue)
//         },
//         "store" => {
//             let store = prompt_user_select_registry_item::<StoreItem>(session, "Store id").await?;
//             println!("{}", store);
//             Ok(NonError::Continue)
//         },
//         _ => Err(Error::InvalidInput),
//     }
// }

// pub fn parse_non_interactive_read_subcommand<'a>(arg_matches: &'a ArgMatches, session: &'a mut Session) -> Pin<Box<dyn Future<Output = Result<NonError, Error>> + 'a>> {
//     pin_action!({
//         let item_type = arg_matches.get_one::<String>("item_type").ok_or(Error::InvalidInput)?;

//         let id = arg_matches.get_one::<String>("id").ok_or(Error::InvalidInput)?;
//         let id = Uuid::parse_str(id)?;

//         match &item_type[..] {
//             "material" => {
//                 let material = session.read::<MaterialItem>(&id).ok_or(Error::InvalidInput)?;
//                 println!("{}", material);
//                 Ok(NonError::Continue)
//             },
//             "store" => {
//                 let store = session.read::<StoreItem>(&id).ok_or(Error::InvalidInput)?;
//                 println!("{}", store);
//                 Ok(NonError::Continue)
//             },
//             _ => Err(Error::InvalidInput)
//         }
//     })
// }
