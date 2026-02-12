use std::str::FromStr;

use business_planner::api::{
    graphs::{Graph, ProductionLine, Recipe}, item::{IngredientItem, MaterialItem, StoreItem}, session::Session
};
use clap::{ArgMatches, Command};
use strum::IntoEnumIterator;

use crate::{
    Error, NonError,
    utils::{IngredientArg, ItemTypes, MaterialArg, Menu, PlannerResult, ProductionLineArg, RecipeArg, StoreArg, prompt_select_command, prompt_user_select_graph, prompt_user_select_registry_item},
};

pub struct DeleteMenu {}

impl Menu for DeleteMenu {
    type SubcommandsEnum = ItemTypes;

    fn get_command() -> Command {
        Command::new("delete")            
            .subcommands(Self::get_subcommands())
    }
    
    fn get_subcommands() -> impl Iterator<Item = Command> {
        [
            Command::new("production-line").visible_alias("line").args(ProductionLineArg::with_alias()),
            Command::new("store").args(StoreArg::with_alias()),
            Command::new("recipe").args(RecipeArg::with_alias()),
            Command::new("ingredient").args(IngredientArg::with_alias()),
            Command::new("material").args(MaterialArg::with_alias()),
        ].into_iter()
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
        let (item_type, subcommand_arg_matches) = arg_matches.subcommand().expect("Clap to have filtered off invalid input");
        let item_type = Self::SubcommandsEnum::from_str(item_type)?;

        match item_type {
            Self::SubcommandsEnum::ProductionLine => {
                let id = ProductionLineArg::parse_arg_matches(subcommand_arg_matches, session)?;
                match session.delete_graph::<ProductionLine>(&id) {
                    Some(production_line) => {
                        println!("Deleted production line \"{}\" ({})", production_line.get_name(), id);
                    },
                    None => Err(Error::NotFound(format!("production line with id {}", id)))?,
                };
            },
            Self::SubcommandsEnum::Store => {
                let id = StoreArg::parse_arg_matches(subcommand_arg_matches, session)?;
                match session.delete::<StoreItem>(&id) {
                    Some(store) => {
                        println!("Deleted store \"{}\" ({})", store.get_name(), id);
                    },
                    None => Err(Error::NotFound(format!("store with id {}", id)))?,
                };
            },
            Self::SubcommandsEnum::Recipe => {
                let id = RecipeArg::parse_arg_matches(subcommand_arg_matches, session)?;
                match session.delete_graph::<Recipe>(&id) {
                    Some(recipe) => {
                        println!("Deleted recipe \"{}\" ({})", recipe.get_name(), id);
                    },
                    None => Err(Error::NotFound(format!("recipe with id {}", id)))?,
                };
            },
            Self::SubcommandsEnum::Ingredient => {
                let id = IngredientArg::parse_arg_matches(subcommand_arg_matches, session)?;
                match session.delete::<IngredientItem>(&id) {
                    Some(ingredient) => {
                        println!("Deleted ingredient \"{}\" ({})", ingredient.get_name(), id);
                    },
                    None => Err(Error::NotFound(format!("ingredient with id {}", id)))?,
                };
            },
            Self::SubcommandsEnum::Material => {
                let id = MaterialArg::parse_arg_matches(subcommand_arg_matches, session)?;
                match session.delete::<MaterialItem>(&id) {
                    Some(material) => {
                        println!("Deleted material \"{}\" ({})", material.get_name(), id);
                    },
                    None => Err(Error::NotFound(format!("material with id {}", id)))?,
                };
            },
        }
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
