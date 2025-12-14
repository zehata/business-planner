
use business_planner::api::{registry::{Material, Store}, session::Session};
use clap::{ArgMatches, Command};
use uuid::Uuid;

use crate::{Error, NonError, registry::{TakesRegistryItemId, retrying_prompt_uuid}};

pub fn get_delete_subcommand() -> Command {
    Command::new("delete")
        .no_binary_name(true)
        .takes_registry_item_id_arg()
}

pub async fn parse_interactive_delete_subcommand(command: &str, session: &mut Session) -> Result<NonError, Error> {
    let id = retrying_prompt_uuid()?;

    match command {
        "material" => {
            session.delete::<Material>(&id);
            Ok(NonError::Continue)
        },
        "store" => {
            session.delete::<Store>(&id);
            Ok(NonError::Continue)
        },
        _ => Err(Error::InvalidInput),
    }
}

pub async fn parse_non_interactive_delete_subcommand(arg_matches: &ArgMatches, session: &mut Session) -> Result<NonError, Error> {
    let item_type = arg_matches.get_one::<String>("item_type").ok_or(Error::InvalidInput)?;

    let id = arg_matches.get_one::<String>("id").ok_or(Error::InvalidInput)?;
    let id = Uuid::parse_str(id)?;
    
    match &item_type[..] {
        "material" => {
            session.delete::<Material>(&id);
            Ok(NonError::Continue)
        },
        "store" => {
            session.delete::<Store>(&id);
            Ok(NonError::Continue)
        },
        _ => Err(Error::InvalidInput)
    }
}