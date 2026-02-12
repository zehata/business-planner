use business_planner::api::{item::MaterialItem, session::Session};
use clap::{Arg, ArgMatches, Command};
use inquire::{Select, Text};
use uuid::Uuid;

use crate::{
    Error, NonError,
    utils::{Menu, NoSubcommands, PlannerResult},
};

pub struct UpdateMaterialMenu {}

impl Menu for UpdateMaterialMenu {
    type SubcommandsEnum = NoSubcommands;

    fn get_command() -> Command {
        Command::new("material")
            .arg(Arg::new("by_id").long("by_id").required(true))
            .arg(Arg::new("name").long("name"))
    }

    async fn interactive(session: &mut Session) -> PlannerResult {
        let (material_id, mut material) = {
            let materials = session.list::<MaterialItem>().collect::<Vec<_>>();
            let material_names = materials
                .iter()
                .map(|(_, material)| material)
                .collect();
            let selected_option = Select::new("Select", material_names)
                .raw_prompt_skippable()?
                .ok_or(Error::UserCancelled)?;
            let (material_id, _) = *materials
                .get(selected_option.index)
                .expect("User cannot select if subcommands is empty");

            (
                material_id.to_owned(),
                session
                    .read::<MaterialItem>(material_id)
                    .ok_or(Error::InvalidInput)?
                    .to_owned(),
            )
        };

        let unchanged_name_hint = material.get_name();
        let name = Text::new("name")
            .with_help_message(&format!(
                "Material name. Leave empty to keep unchanged {}",
                unchanged_name_hint
            ))
            .prompt_skippable()?;
        if let Some(name) = name {
            material.set_name(&name);
        }

        session.update(&material_id, material);

        Ok(NonError::Continue)
    }

    async fn non_interactive(arg_matches: &ArgMatches, session: &mut Session) -> PlannerResult {
        let string = arg_matches
            .get_one::<String>("by_id")
            .ok_or(Error::InvalidInput)?;
        let id = Uuid::parse_str(string)?;
        let mut material = get_material_by_uuid(session, &id)
            .ok_or(Error::InvalidInput)?
            .to_owned();

        if let Some(name) = arg_matches.get_one::<String>("material_name") {
            material.set_name(name);
        };

        session.update(&id, material);

        Ok(NonError::Continue)
    }
}

pub fn get_material_by_uuid<'a>(session: &'a mut Session, uuid: &Uuid) -> Option<&'a MaterialItem> {
    session.read::<MaterialItem>(uuid)
}

// pub fn get_update_material_interactive_subcommand(session: &mut Session) -> Vec<String> {
//     session.list::<MaterialItem>()
// }

// pub async fn parse_update_material_interactive_subcommand(command: &str, session: &mut Session) -> Result<NonError, Error> {
//     let id = Uuid::parse_str(command)?;
//     let mut material = session.read::<MaterialItem>(&id).ok_or(Error::InvalidInput)?.to_owned();

//     let unchanged_name_hint = match material.get_name() {
//         Some(name) => &format!("({})", name),
//         None => "",
//     };
//     let name = Text::new("name")
//         .with_help_message(&format!("Material name. Leave empty to keep unchanged {}", unchanged_name_hint))
//         .prompt_skippable()?;
//     if let Some(name) = name {
//         material.set_name(&name);
//     }

//     session.update(&id, material);

//     Ok(NonError::Continue)
// }

#[cfg(test)]
mod test {
    // use business_planner::api::session::create_session;

    // use crate::{error::Error, registry::update::get_update_commands};

    // use super::*;

    // async fn parse(test_buffer: &str, session: &mut Session) -> Result<NonError, Error> {
    //     let arg_matches = get_command_matches(test_buffer, get_update_command())?;
    //     let user_command = UserCommand::NonInteractive(arg_matches);
    //     parse_update_material_non_interactive_subcommand(user_command, session).await
    // }

    // #[tokio::test]
    // async fn test_update_material() {
    //     let mut session = create_session();
    //     let uuid = session.create(MaterialItem::new());

    //     let buffer = format!("--by_id {} --name \"test name\"", uuid);
    //     let result = parse(&buffer, &mut session).await;
    //     result.expect("update command should run successfully");
    //     let material = session.read::<MaterialItem>(&uuid);
    //     assert_eq!(material.expect("material should exist").get_name().expect("name should be set"), "test name")
    // }
}
