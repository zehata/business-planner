use business_planner::api::session::Session;
use clap::{ArgMatches, Command};

use crate::{Error, NonError, registry::{update::{material::{get_update_material_interactive_subcommand, get_update_material_subcommand, parse_update_material_interactive_subcommand, parse_update_material_non_interactive_subcommand}, store::{get_update_store_interactive_subcommand, get_update_store_subcommand, parse_update_store_interactive_subcommand, parse_update_store_non_interactive_subcommand}}}, shells::interactive};

pub mod material;
pub mod store;

pub fn get_read_subcommand() -> Command {
    Command::new("read")
        .no_binary_name(true)
        .subcommands([
            get_update_material_subcommand(),
            get_update_store_subcommand(),
        ])
}

pub async fn parse_interactive_update_subcommand(command: &str, session: &mut Session) -> Result<NonError, Error> {
    match command {
        "material" => {
            interactive::shell(
                get_update_material_interactive_subcommand(session),
                parse_update_material_interactive_subcommand,
                session
            ).await
        },
        "store" => {
            interactive::shell(
                get_update_store_interactive_subcommand(),
                parse_update_store_interactive_subcommand,
                session
            ).await
        },
        _ => Err(Error::InvalidInput),
    }
}

pub async fn parse_non_interactive_update_subcommand(arg_matches: &ArgMatches, session: &mut Session) -> Result<NonError, Error> {
    match arg_matches.subcommand() {
        Some(("material", arg_matches)) => parse_update_material_non_interactive_subcommand(arg_matches, session).await,
        Some(("store", arg_matches)) => parse_update_store_non_interactive_subcommand(arg_matches, session).await,
        _ => Err(Error::InvalidInput)
    }
}