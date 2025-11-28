use business_planner::api::{registry::RegistryItemType, session::Session};
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
            session.create(RegistryItemType::Material);
            Ok(NonError::Continue)
        },
        "store" => {
            session.create(RegistryItemType::Store);
            Ok(NonError::Continue)
        },
        _ => Err(Error::InvalidInput),
    }
}

pub async fn parse_non_interactive_create_subcommand(arg_matches: &ArgMatches, session: &mut Session) -> Result<NonError, Error> {
    match arg_matches.subcommand() {
        Some(("material", _)) => {
            session.create(RegistryItemType::Material);
            Ok(NonError::Continue)
        },
        Some(("store", _)) => {
            session.create(RegistryItemType::Store);
            Ok(NonError::Continue)
        },
        _ => Err(Error::InvalidInput),
    }
}