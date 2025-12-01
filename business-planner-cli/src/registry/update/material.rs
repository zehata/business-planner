use business_planner::api::{registry::Material, session::Session};
use clap::{Arg, ArgMatches, Command};
use inquire::Text;
use uuid::Uuid;

use crate::{Error, NonError};

pub fn get_update_material_subcommand() -> Command {
    Command::new("material")
        .no_binary_name(true)
        .arg(
            Arg::new("by_id")
                .long("by_id")
                .required(true)
        )
        .arg(
            Arg::new("name")
                .long("name")
        )
}

pub fn get_material_by_uuid<'a>(session: &'a mut Session, uuid: &Uuid) -> Option<&'a mut Material> {
    session.read::<Material>(uuid)
}

pub fn select_material<'a>(session: &'a mut Session, arg_matches: &ArgMatches) -> Option<&'a mut Material> {
    if
        let Some(uuid) = arg_matches.get_one::<String>("by_id") &&
        let Ok(uuid) = Uuid::parse_str(uuid)
    {
        return get_material_by_uuid(session, &uuid)
    }
    None
}

pub fn get_update_material_interactive_subcommand(session: &mut Session) -> Vec<String> {
    session.list::<Material>()
}

pub async fn parse_update_material_interactive_subcommand(command: &str, session: &mut Session) -> Result<NonError, Error> {
    let Some(material) = session.read::<Material>(&Uuid::parse_str(command)?) else {
        return Err(Error::InvalidInput)
    };

    let unchanged_name_hint = match material.get_name() {
        Some(name) => &format!("({})", name),
        None => "",
    };
    let name = Text::new("name")
        .with_help_message(&format!("Material name. Leave empty to keep unchanged {}", unchanged_name_hint))
        .prompt_skippable()?;
    if let Some(name) = name {
        material.set_name(&name);
    }

    Ok(NonError::Continue)
}

pub async fn parse_update_material_non_interactive_subcommand(arg_matches: &ArgMatches, session: &mut Session) -> Result<NonError, Error> {
    let Some(material) = select_material(session, arg_matches) else {
        return Err(Error::InvalidInput)
    };

    if let Some(name) = arg_matches.get_one::<String>("name") {
        material.set_name(name);
    };

    Ok(NonError::Continue)
}


#[cfg(test)]
mod test {
    use business_planner::api::session::create_session;

    use crate::{error::Error, shells::non_interactive::parse_buffer};

    use super::*;

    async fn parse(test_buffer: &str, session: &mut Session) -> Result<NonError, Error> {
        parse_buffer(
            test_buffer,
            get_update_material_subcommand(),
            parse_update_material_non_interactive_subcommand,
            session
        ).await
    }

    #[tokio::test]
    async fn test_update_material() {
        let mut session = create_session();
        let uuid = session.create::<Material>();
        
        let buffer = format!("--by_id {} --name \"test name\"", uuid);
        let result = parse(&buffer, &mut session).await;
        result.expect("update command should run successfully");
        let material = session.read::<Material>(&uuid);
        assert_eq!(material.expect("material should exist").get_name().expect("name should be set"), "test name")
    }
}