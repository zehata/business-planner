use std::io::{Write, stdout};

use business_planner::api::session::Session;
use clap::{Arg, ArgMatches, Command};
use inquire::Text;
use uuid::Uuid;

use crate::{Error, NonError, registry::{create::{get_create_subcommand, parse_interactive_create_subcommand, parse_non_interactive_create_subcommand}, read::{get_read_subcommand, parse_interactive_read_subcommand, parse_non_interactive_read_subcommand}}, shells::interactive};

pub mod create;
pub mod read;
pub mod update;
pub mod delete;
pub mod list;

pub fn get_registry_subcommand() -> Command {
    Command::new("registry")
        .no_binary_name(true)
        .subcommand_required(true)
        .subcommands([
            get_create_subcommand(),
            get_read_subcommand(),
        ])
}

pub async fn parse_interactive_registry_subcommand(command: &str, session: &mut Session) -> Result<NonError, Error> {
    match command {
        "create" => {
            interactive::shell(
                get_registry_item_types(),
                parse_interactive_create_subcommand,
                session,
            ).await
        },
        "read" => {
            interactive::shell(
                get_registry_item_types(),
                parse_interactive_read_subcommand,
                session,
            ).await
        },
        _ => Err(Error::InvalidInput),
    }
}

pub async fn parse_non_interactive_registry_subcommand(arg_matches: &ArgMatches, session: &mut Session) -> Result<NonError, Error> {
    match arg_matches.subcommand() {
        Some(("create", arg_matches)) => {
            parse_non_interactive_create_subcommand(arg_matches, session).await
        },
        Some(("read", _)) => {
            parse_non_interactive_read_subcommand(arg_matches, session).await
        },
        _ => Err(Error::InvalidInput),
    }
}

trait TakesRegistryItemType {
    fn takes_registry_item_type_arg(self) -> Command;
}

const REGISTRY_ITEMS: [&str; 2] = ["material", "store"];

pub fn get_registry_item_types() -> Vec<String> {
    REGISTRY_ITEMS.into_iter().map(|item_type| {
        item_type.to_string()
    }).collect()
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
        self.arg(
            Arg::new("id")
                .required(true)
        )
    }
}

pub fn retrying_prompt_uuid() -> Result<Uuid, Error> {
    println!("item id to read? (Esc or Ctrl+C to cancel)");
    stdout().flush().expect("Failed to print to stdout");

    let mut uuid: Option<Uuid> = None;
    while uuid.is_none() {
        match Text::new("id:").prompt() {
            Ok(input) => {
                match Uuid::parse_str(&input) {
                    Ok(parsed) => { uuid = Some(parsed) },
                    Err(_) => continue,
                }
            },
            _ => {
                return Err(Error::UserCancelled)
            },
        };

        println!("Input is invalid");
        stdout().flush().expect("Failed to print to stdout");
    };

    Ok(uuid.expect("Loop only ends when uuid is not None"))
}