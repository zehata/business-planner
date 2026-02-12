use std::str::FromStr;

use business_planner::api::{
    graphs::{ProductionLine, Recipe}, item::{IngredientItem, MaterialItem, StoreItem}, session::Session
};
use clap::{ArgMatches, Command};
use strum::IntoEnumIterator;

use crate::{
    Error, NonError,
    utils::{ItemTypes, Menu, PlannerResult, print_list, prompt_select_command},
};

pub struct ListMenu {}

impl ListMenu {
    fn list_items(selected_command: &ItemTypes, session: &mut Session) -> Result<NonError, Error> {        
        let items = match selected_command {
            ItemTypes::ProductionLine => session.list_graphs::<ProductionLine>().collect::<Vec<_>>(),
            ItemTypes::Store => session.list::<StoreItem>().collect::<Vec<_>>(),
            ItemTypes::Recipe => session.list_graphs::<Recipe>().collect::<Vec<_>>(),
            ItemTypes::Ingredient => session.list::<IngredientItem>().collect::<Vec<_>>(),
            ItemTypes::Material => session.list::<MaterialItem>().collect::<Vec<_>>(),
        };
        print_list(items);
        Ok(NonError::Continue)
    }
}

impl Menu for ListMenu {
    type SubcommandsEnum = ItemTypes;

    fn get_command() -> Command {
        Command::new("list")
            .subcommands(Self::get_subcommands())
    }

    fn get_subcommands() -> impl Iterator<Item = Command> {
        [
            Command::new("production-line").visible_alias("line"),
            Command::new("store"),
            Command::new("recipe"),
            Command::new("ingredient"),
            Command::new("material"),
        ].into_iter()
    }

    async fn interactive(session: &mut Session) -> PlannerResult {
        let subcommands = Self::SubcommandsEnum::iter();
        let selected_subcommand = prompt_select_command(subcommands)?;

        Self::list_items(&selected_subcommand, session)
    }

    async fn non_interactive(arg_matches: &ArgMatches, session: &mut Session) -> PlannerResult {
        let (selected_subcommand, _) = arg_matches.subcommand().expect("Clap to have filtered off invalid input");
        let selected_subcommand = Self::SubcommandsEnum::from_str(selected_subcommand)?;

        Self::list_items(&selected_subcommand, session)
    }
}

// pub fn get_list_subcommand() -> Command {
//     Command::new("create")
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
