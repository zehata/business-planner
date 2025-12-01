
use business_planner::api::{registry::{Material, Store}, session::Session};
use clap::{ArgMatches, Command};
use uuid::Uuid;

use crate::{Error, NonError, registry::{TakesRegistryItemId, retrying_prompt_uuid}};

pub fn get_read_subcommand() -> Command {
    Command::new("read")
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
    let Some(item_type) = arg_matches.get_one::<String>("item_type") else {
        return Err(Error::InvalidInput)
    };

    let Some(id) = arg_matches.get_one::<String>("id") else {
        return Err(Error::InvalidInput)
    };
    let Ok(id) = Uuid::parse_str(id) else {
        return Err(Error::InvalidInput)
    };
    
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