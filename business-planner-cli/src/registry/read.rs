use business_planner::api::{item::{Item, MaterialItem, StoreItem}, session::Session};
use clap::{ArgMatches, Command};
use inquire::{InquireError, Select};
use uuid::Uuid;

use crate::{Error, NonError, registry::TakesRegistryItemId};

pub fn get_read_subcommand() -> Command {
    Command::new("read")
        .no_binary_name(true)
        .takes_registry_item_id_arg()
}

pub async fn prompt_user_select_registry_item<'a, T>(session: &'a mut Session, message: &str) -> Result<&'a Uuid, InquireError>
    where
        T: 'a + Item,
    {

    let materials = session.list_names::<T>();
    let material_names = materials.iter().map(|(uuid, _)| {
        uuid.to_string()
    }).collect::<Vec<_>>();
    let selection = Select::new(message, material_names).raw_prompt()?;
    let (id, _) = materials.get(selection.index).unwrap();
    Ok(id)
}

pub async fn parse_interactive_read_subcommand(command: &str, session: &mut Session) -> Result<NonError, Error> {
    match command {
        "material" => {
            let material = prompt_user_select_registry_item::<MaterialItem>(session, "Material id").await?;
            println!("{}", material);
            Ok(NonError::Continue)
        },
        "store" => {
            let store = prompt_user_select_registry_item::<StoreItem>(session, "Store id").await?;
            println!("{}", store);
            Ok(NonError::Continue)
        },
        _ => Err(Error::InvalidInput),
    }
}

pub async fn parse_non_interactive_read_subcommand(arg_matches: &ArgMatches, session: &mut Session) -> Result<NonError, Error> {
    let item_type = arg_matches.get_one::<String>("item_type").ok_or(Error::InvalidInput)?;

    let id = arg_matches.get_one::<String>("id").ok_or(Error::InvalidInput)?;
    let id = Uuid::parse_str(id)?;
    
    match &item_type[..] {
        "material" => {
            let material = session.read::<MaterialItem>(&id).ok_or(Error::InvalidInput)?;
            println!("{}", material);
            Ok(NonError::Continue)
        },
        "store" => {
            let store = session.read::<StoreItem>(&id).ok_or(Error::InvalidInput)?;
            println!("{}", store);
            Ok(NonError::Continue)
        },
        _ => Err(Error::InvalidInput)
    }
}