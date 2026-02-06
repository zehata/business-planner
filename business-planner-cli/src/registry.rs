use std::io::{Write, stdout};

use clap::{Arg, Command};
use enum_map::{Enum, EnumMap, enum_map};
use inquire::Text;
use strum_macros::{Display, EnumIter, EnumString};
use uuid::Uuid;

use crate::{
    Error,
    registry::{
        create::CreateRegistryItemMenu, delete::DeleteRegistryItemMenu, list::ListRegistryItemMenu,
        read::ReadRegistryItemMenu, update::UpdateRegistryItemMenu,
    },
    utils::{Menu, Submenu},
};

mod create;
mod delete;
mod list;
mod read;
mod update;

#[derive(Debug, Enum, EnumString)]
pub enum RegistryCommands {
    Create,
    Read,
    Update,
    Delete,
}

#[derive(Debug, Display, Enum, EnumString)]
pub enum RegistryMenu {
    Create,
    Read,
    Update,
    Delete,
    List,
}

impl Menu for RegistryMenu {
    fn get_command() -> Command {
        Command::new("registry")
            .no_binary_name(true)
            .subcommand_required(true)
            .subcommands(Self::get_subcommands())
    }

    fn get_submenus() -> EnumMap<Self, Submenu> {
        enum_map! {
            Self::Create => CreateRegistryItemMenu::get_submenu(),
            Self::Read => ReadRegistryItemMenu::get_submenu(),
            Self::Update => UpdateRegistryItemMenu::get_submenu(),
            Self::Delete => DeleteRegistryItemMenu::get_submenu(),
            Self::List => ListRegistryItemMenu::get_submenu(),
        }
    }
}

#[derive(Clone, Copy, Debug, Display, EnumIter, EnumString)]
pub enum RegistryItemTypes {
    Material,
    Ingredient,
    Store,
}

// impl BusinessPlannerCommand for RegistryCommands {
//     fn get_command(is_interactive: bool) -> Command {
//         Command::new("registry")
//             .no_binary_name(true)
//             .subcommand_required(true)
//             .subcommands(Self::get_subcommands(is_interactive).into_values())
//     }

//     fn get_subcommands(is_interactive: bool) -> EnumMap<Self, Command> {
//         enum_map! {
//             Self::Create => get_create_command(is_interactive),
//             Self::Read => get_read_command(is_interactive),
//             Self::Update => get_update_command(is_interactive),
//             Self::Delete => get_delete_command(is_interactive),
//         }
//     }

//     fn parse_command(user_command: UserCommand) -> Result<NonError, Error> {
//         match user_command {
//             UserCommand::Interactive(command) => {
//                 let subcommand = Self::from_str(command)?;
//                 match subcommand {
//                     Self::Create => parse_interactive_create_subcommand,
//                     Self::Read => parse_interactive_read_subcommand,
//                 }
//             },
//             UserCommand::NonInteractive(command, arg_matches) => {

//             }
//         }
//         let subcommand = Self::from_str(command)?;
//         Ok(match subcommand {
//             Self::Create => CommandParsers::new(
//                 parse_interactive_create_subcommand,
//                 parse_non_interactive_create_subcommand
//             ),
//             Self::Read => CommandParsers::new(
//                 parse_interactive_read_subcommand,
//                 parse_non_interactive_read_subcommand
//             ),
//             Self::Update => CommandParsers::new(
//                 parse_interactive_update_subcommand,
//                 parse_non_interactive_update_subcommand
//             ),
//             Self::Delete => CommandParsers::new(
//                 parse_interactive_delete_subcommand,
//                 parse_non_interactive_delete_subcommand,
//             ),
//         })
//     }
// }

trait TakesRegistryItemType {
    fn takes_registry_item_type_arg(self) -> Command;
}

const REGISTRY_ITEMS: [&str; 2] = ["material", "store"];

pub fn get_registry_item_types() -> Vec<String> {
    REGISTRY_ITEMS
        .into_iter()
        .map(|item_type| item_type.to_string())
        .collect()
}

impl TakesRegistryItemType for Command {
    fn takes_registry_item_type_arg(self) -> Command {
        self.arg(
            Arg::new("item_type")
                .required(true)
                .num_args(1)
                .value_parser(REGISTRY_ITEMS),
        )
    }
}

trait TakesRegistryItemId {
    fn takes_registry_item_id_arg(self) -> Command;
}

impl TakesRegistryItemId for Command {
    fn takes_registry_item_id_arg(self) -> Command {
        self.arg(Arg::new("id").required(true))
    }
}

pub fn retrying_prompt_uuid() -> Result<Uuid, Error> {
    println!("item id to read? (Esc or Ctrl+C to cancel)");
    stdout().flush().expect("Failed to print to stdout");

    let mut uuid: Option<Uuid> = None;
    while uuid.is_none() {
        match Text::new("id:").prompt() {
            Ok(input) => match Uuid::parse_str(&input) {
                Ok(parsed) => uuid = Some(parsed),
                Err(_) => continue,
            },
            _ => return Err(Error::UserCancelled),
        };

        println!("Input is invalid");
        stdout().flush().expect("Failed to print to stdout");
    }

    Ok(uuid.expect("Loop only ends when uuid is not None"))
}
