use business_planner::api::{registry::{Material, RegistryItem, Store}, session::Session};
use clap::{ArgMatches, Command};

use crate::{Error, NonError, registry::TakesRegistryItemType};

pub fn get_create_subcommand() -> Command {
    Command::new("create")
        .no_binary_name(true)
        .takes_registry_item_type_arg()
}

pub async fn parse_interactive_create_subcommand(command: &str, session: &mut Session) -> Result<NonError, Error> {
    match command {
        "material" => {
            Material::create_in_session(Material{}, session);
            Ok(NonError::Continue)
        },
        "store" => {
            Store::create_in_session(Store{ usage_data_source: None }, session);
            Ok(NonError::Continue)
        },
        _ => Err(Error::InvalidInput),
    }
}

pub async fn parse_non_interactive_create_subcommand(arg_matches: &ArgMatches, session: &mut Session) -> Result<NonError, Error> {
    match arg_matches.subcommand() {
        Some(("material", _)) => {
            Material::create_in_session(Material{}, session);
            Ok(NonError::Continue)
        },
        Some(("store", _)) => {
            Store::create_in_session(Store{ usage_data_source: None }, session);
            Ok(NonError::Continue)
        },
        _ => Err(Error::InvalidInput),
    }
}