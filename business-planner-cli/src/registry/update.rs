use business_planner::api::{registry::{RegistryItem, RegistryItemType}, session::Session};
use clap::{Arg, ArgMatches, Command};
use uuid::Uuid;

use crate::{Error, NonError, registry::retrying_prompt_uuid};

pub fn get_read_subcommand() -> Command {
    Command::new("read")
        .no_binary_name(true)
        .subcommands([
            Command::new("material")
                .args([
                    Arg::new("name"),
                ]),
            Command::new("store")
                .args([
                    Arg::new("name"),
                ]),
        ])
}

pub async fn parse_interactive_update_subcommand(command: &str, session: &mut Session) -> Result<NonError, Error> {
    let id = retrying_prompt_uuid()?;
    match command {
        "material" => {
            let Some(RegistryItem::Material(material)) = session.read(RegistryItemType::Material, id) else {
                return Err(Error::InvalidInput)
            };
            Ok(NonError::Continue)
        },
        "store" => {
            let Some(RegistryItem::Store(store)) = session.read(RegistryItemType::Store, id) else {
                return Err(Error::InvalidInput)
            };
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
            let Some(RegistryItem::Material(material)) = session.read(RegistryItemType::Material, id) else {
                return Err(Error::InvalidInput)
            };
            material.set_name("test");
            Ok(NonError::Continue)
        },
        "store" => {
            let Some(RegistryItem::Store(store)) = session.read(RegistryItemType::Store, id) else {
                return Err(Error::InvalidInput)
            };
            store.set_name("test");
            Ok(NonError::Continue)
        },
        _ => Err(Error::InvalidInput)
    }
}