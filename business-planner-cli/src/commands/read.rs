use std::str::FromStr;

use business_planner::api::{
    graphs::{ProductionLine, Recipe}, item::{IngredientItem, MaterialItem, StoreItem}, session::Session
};
use clap::{ArgMatches, Command};
use strum::IntoEnumIterator;

use crate::{NonError, utils::{IngredientArg, ItemTypes, MaterialArg, Menu, PlannerResult, ProductionLineArg, RecipeArg, StoreArg, prompt_select_command, prompt_user_select_graph, prompt_user_select_registry_item}};

pub struct ReadMenu {}

impl Menu for ReadMenu {
    type SubcommandsEnum = ItemTypes;

    fn get_command() -> Command {
        Command::new("read")
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
                let production_line_id = prompt_user_select_graph::<ProductionLine>(session, "Select Production Line")
                    .await?
                    .to_owned();
                println!("{}", session.read_graph::<ProductionLine>(&production_line_id).expect(""))
            },
            Self::SubcommandsEnum::Store => {
                let store_id = prompt_user_select_registry_item::<StoreItem>(session, "Select Store")
                    .await?;
                println!("{}", session.read::<StoreItem>(store_id).expect(""));
            },
            Self::SubcommandsEnum::Ingredient => {
                let ingredient_id =
                    prompt_user_select_registry_item::<IngredientItem>(session, "Select Ingredient")
                        .await?;
                println!(
                    "{}",
                    session.read::<IngredientItem>(ingredient_id).expect("")
                );
            },
            Self::SubcommandsEnum::Recipe => {
                let recipe_id = prompt_user_select_graph::<Recipe>(session, "Select Recipe")
                    .await?
                    .to_owned();
                println!("{}", session.read_graph::<Recipe>(&recipe_id).expect(""))
            },
            Self::SubcommandsEnum::Material => {
                let material_id =
                    prompt_user_select_registry_item::<MaterialItem>(session, "Select Material")
                        .await?;
                println!("{}", session.read::<MaterialItem>(material_id).expect(""));
            },
        };
        Ok(NonError::Continue)
    }

    async fn non_interactive(arg_matches: &ArgMatches, session: &mut Session) -> PlannerResult {
        let (item_type, subcommand_arg_matches) = arg_matches.subcommand().expect("Clap to have filtered off invalid input");
        let item_type = Self::SubcommandsEnum::from_str(item_type)?;

        match item_type {
            Self::SubcommandsEnum::ProductionLine => {
                let id = ProductionLineArg::parse_arg_matches(subcommand_arg_matches, session)?.expect("Clap to have filtered off missing argument");
                println!("{}", session.read_graph::<ProductionLine>(&id).expect(""));
            },
            Self::SubcommandsEnum::Store => {
                let id = StoreArg::parse_arg_matches(subcommand_arg_matches, session)?;
                println!("{}", session.read::<StoreItem>(&id).expect(""));
            },
            Self::SubcommandsEnum::Recipe => {
                let id = RecipeArg::parse_arg_matches(subcommand_arg_matches, session)?.expect("Clap to have filtered off missing argument");
                println!("{}", session.read_graph::<Recipe>(&id).expect(""));
            },
            Self::SubcommandsEnum::Ingredient => {
                let id = IngredientArg::parse_arg_matches(subcommand_arg_matches, session)?;
                println!("{}", session.read::<IngredientItem>(&id).expect(""));
            },
            Self::SubcommandsEnum::Material => {
                let id = MaterialArg::parse_arg_matches(subcommand_arg_matches, session)?;
                println!("{}", session.read::<MaterialItem>(&id).expect(""));
            },
        }
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
