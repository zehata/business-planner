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

pub fn retrying_prompt_uuid() -> Uuid {
    let mut uuid = Uuid::parse_str("");
    while uuid.is_err() {
        let Ok(user_input) = Text::new("id").prompt() else {
            continue;
        };
        uuid = Uuid::parse_str(&user_input);
    };
    uuid.expect("Loop does not end until uuid is valid")
}

pub async fn parse_interactive_update_subcommand(command: &str, session: &mut Session) -> Result<NonError, Error> {
    let id = retrying_prompt_uuid();
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

pub async fn parse_non_interactive_update_subcommand(arg_matches: &ArgMatches, session: &mut Session) -> Result<NonError, Error> {
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
            Material::update_in_session(id, Material{}, session);
            Ok(NonError::Continue)
        },
        "store" => {
            Store::update_in_session(id, Store { usage_data_source: None }, session);
            Ok(NonError::Continue)
        },
        _ => Err(Error::InvalidInput)
    }
}