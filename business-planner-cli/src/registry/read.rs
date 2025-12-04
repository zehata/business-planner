use business_planner::api::{registry::{Material, RegistryItem, Store}, session::Session};
use clap::{ArgMatches, Command};
use inquire::{InquireError, Select};
use uuid::Uuid;

use crate::{Error, NonError, registry::TakesRegistryItemId};

pub fn get_read_subcommand() -> Command {
    Command::new("read")
        .no_binary_name(true)
        .takes_registry_item_id_arg()
}

pub async fn prompt_user_select_registry_item<'a, T>(session: &'a Session, message: &str) -> Result<&'a T, InquireError> where T: 'a + RegistryItem<Item = T> {
    let materials = session.list_names::<T>();
    let material_names = materials.iter().map(|(uuid, _)| {
        uuid.to_string()
    }).collect::<Vec<_>>();
    let selection = Select::new(message, material_names).raw_prompt()?;
    let (id, _) = materials.get(selection.index).unwrap();
    Ok(session.read::<T>(id).unwrap())
}

pub async fn parse_interactive_read_subcommand(command: &str, session: &mut Session) -> Result<NonError, Error> {
    match command {
        "material" => {
            let material = prompt_user_select_registry_item::<Material>(session, "Material id").await?;
            println!("{}", material);
            Ok(NonError::Continue)
        },
        "store" => {
            let store = prompt_user_select_registry_item::<Store>(session, "Store id").await?;
            println!("{}", store);
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
            let Some(material) = session.read::<Material>(&id) else {
                return Err(Error::InvalidInput)
            };
            println!("{}", material);
            Ok(NonError::Continue)
        },
        "store" => {
            let Some(store) = session.read::<Store>(&id) else {
                return Err(Error::InvalidInput)
            };
            println!("{}", store);
            Ok(NonError::Continue)
        },
        _ => Err(Error::InvalidInput)
    }
}