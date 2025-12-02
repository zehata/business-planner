use business_planner::api::session::Session;
use clap::{ArgMatches, Command};

use crate::{Error, NonError, registry::{create::{material::{create_material_interactive_subcommand, get_create_material_subcommand, parse_create_material_non_interactive_subcommand}, store::{create_store_interactive_subcommand, get_create_store_subcommand, parse_create_store_non_interactive_subcommand}}}};

pub mod material;
pub mod store;

pub fn get_create_subcommand() -> Command {
    Command::new("create")
        .no_binary_name(true)
        .subcommands([
            get_create_material_subcommand(),
            get_create_store_subcommand(),
        ])
}

pub async fn parse_interactive_create_subcommand(command: &str, session: &mut Session) -> Result<NonError, Error> {
    match command {
        "material" => {
            create_material_interactive_subcommand(session).await
        },
        "store" => {
            create_store_interactive_subcommand(session).await
        },
        _ => Err(Error::InvalidInput),
    }
}

pub async fn parse_non_interactive_create_subcommand(arg_matches: &ArgMatches, session: &mut Session) -> Result<NonError, Error> {
    match arg_matches.subcommand() {
        Some(("material", _)) => {
            parse_create_material_non_interactive_subcommand(arg_matches, session).await
        },
        Some(("store", _)) => {
            parse_create_store_non_interactive_subcommand(arg_matches, session).await
        },
        _ => Err(Error::InvalidInput),
    }
}