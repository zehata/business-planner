use business_planner::api::{
    graphs::{ProductionLine, Recipe}, item::{IngredientItem, MaterialItem, StoreItem}, session::Session
};
use clap::{ArgMatches, Command};
use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::{
    Error, NonError,
    utils::{ItemTypes, Menu, PlannerResult, TakesItemId, prompt_select_command, prompt_user_select_graph, prompt_user_select_registry_item},
};

pub struct DeleteMenu {}

impl Menu for DeleteMenu {
    type SubcommandsEnum = ItemTypes;

    fn get_command() -> Command {
        Command::new("delete")            
            .takes_item_id_arg()
    }

    async fn interactive(session: &mut Session) -> PlannerResult {
        let subcommands = Self::SubcommandsEnum::iter();
        let selected_subcommand = prompt_select_command(subcommands)?;

        match selected_subcommand {
            Self::SubcommandsEnum::ProductionLine => {
                let production_line_id = prompt_user_select_graph::<ProductionLine>(session, "Select recipe").await?.to_owned();
                session.delete_graph::<ProductionLine>(&production_line_id);
            },
            Self::SubcommandsEnum::Ingredient => {
                let ingredient_id =
                    prompt_user_select_registry_item::<IngredientItem>(session, "Ingredient name")
                        .await?
                        .to_owned();
                session.delete::<IngredientItem>(&ingredient_id);
            },
            Self::SubcommandsEnum::Recipe => {
                let recipe_id = prompt_user_select_graph::<Recipe>(session, "Select recipe").await?.to_owned();
                session.delete_graph::<Recipe>(&recipe_id);
            },
            Self::SubcommandsEnum::Store => {
                let store_id = prompt_user_select_registry_item::<StoreItem>(session, "Store name")
                    .await?
                    .to_owned();
                session.delete::<StoreItem>(&store_id);
            },
            Self::SubcommandsEnum::Material => {
                let material_id =
                    prompt_user_select_registry_item::<MaterialItem>(session, "Material name")
                        .await?
                        .to_owned();
                session.delete::<MaterialItem>(&material_id);
            },
        };
        Ok(NonError::Continue)
    }

    async fn non_interactive(arg_matches: &ArgMatches, session: &mut Session) -> PlannerResult {
        let item_type = arg_matches
            .get_one::<Self::SubcommandsEnum>("item_type")
            .ok_or(Error::InvalidInput)?;

        let id = arg_matches
            .get_one::<String>("id")
            .ok_or(Error::InvalidInput)?;
        let id = Uuid::parse_str(id)?;

        match item_type {
            Self::SubcommandsEnum::ProductionLine => {session.delete_graph::<ProductionLine>(&id);},
            Self::SubcommandsEnum::Ingredient => {session.delete::<IngredientItem>(&id);},
            Self::SubcommandsEnum::Recipe => {session.delete_graph::<Recipe>(&id);},
            Self::SubcommandsEnum::Store => {session.delete::<StoreItem>(&id);},
            Self::SubcommandsEnum::Material => {session.delete::<MaterialItem>(&id);},
        };
        Ok(NonError::Continue)
    }
}

// pub fn get_delete_command() -> Command {
//     Command::new("delete")
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
