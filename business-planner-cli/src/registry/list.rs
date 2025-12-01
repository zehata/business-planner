use business_planner::api::{registry::{Material, Store}, session::Session};
use clap::{ArgMatches, Command};

use crate::{Error, NonError, registry::TakesRegistryItemType};

pub fn get_list_subcommand() -> Command {
    Command::new("create")
        .no_binary_name(true)
        .takes_registry_item_type_arg()
}

pub async fn parse_interactive_list_subcommand(command: &str, session: &mut Session) -> Result<NonError, Error> {
    match command {
        "material" => {
            session.list::<Material>();
            Ok(NonError::Continue)
        },
        "store" => {
            session.list::<Store>();
            Ok(NonError::Continue)
        },
        _ => Err(Error::InvalidInput),
    }
}

pub async fn parse_non_interactive_list_subcommand(arg_matches: &ArgMatches, session: &mut Session) -> Result<NonError, Error> {
    match arg_matches.subcommand() {
        Some(("material", _)) => {
            session.list::<Material>();
            Ok(NonError::Continue)
        },
        Some(("store", _)) => {
            session.list::<Store>();
            Ok(NonError::Continue)
        },
        _ => Err(Error::InvalidInput),
    }
}