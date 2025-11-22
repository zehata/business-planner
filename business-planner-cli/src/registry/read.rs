use std::io::{Write, stdout};

use business_planner::api::{registry::{Material, RegistryItem, Store}, session::Session};
use clap::{ArgMatches, Command};
use inquire::Text;
use uuid::Uuid;

use crate::{Error, NonError, registry::{TakesRegistryItemId}};

pub fn get_read_subcommand() -> Command {
    Command::new("read")
        .no_binary_name(true)
        .takes_registry_item_id_arg()
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

pub async fn parse_interactive_read_subcommand(command: &str, session: &mut Session) -> Result<NonError, Error> {
    let id = retrying_prompt_uuid()?;
    match command {
        "material" => {
            Material::read_in_session(id, session);
            Ok(NonError::Continue)
        },
        "store" => {
            Store::read_in_session(id, session);
            Ok(NonError::Continue)
        },
        _ => Err(Error::InvalidInput),
    }
}

pub async fn parse_non_interactive_read_subcommand(arg_matches: &ArgMatches, session: &mut Session) -> Result<NonError, Error> {
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
            Material::read_in_session(id, session);
            Ok(NonError::Continue)
        },
        "store" => {
            Store::read_in_session(id, session);
            Ok(NonError::Continue)
        },
        _ => Err(Error::InvalidInput)
    }
}