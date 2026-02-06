use business_planner::api::{item::MaterialItem, session::Session};
use clap::{Arg, ArgMatches, Command};
use enum_map::Enum;
use inquire::Text;
use strum_macros::{Display, EnumString};

use crate::{
    Error, NonError,
    utils::{Menu, PlannerResult},
};

#[derive(Debug, Display, Enum, EnumString)]
pub enum CreateMaterialMenu {}

impl Menu for CreateMaterialMenu {
    fn get_command() -> Command {
        Command::new("material")
            .no_binary_name(true)
            .arg(Arg::new("name").long("name"))
    }

    async fn interactive(session: &mut Session) -> PlannerResult {
        create_material_interactive(session).await
    }

    async fn non_interactive(arg_matches: &ArgMatches, session: &mut Session) -> PlannerResult {
        create_material_non_interactive(arg_matches, session).await
    }
}

pub async fn create_material_interactive(session: &mut Session) -> Result<NonError, Error> {
    let mut material = MaterialItem::new();

    if let Some(name) = Text::new("name")
        .with_help_message("Material name.")
        .prompt_skippable()?
    {
        material.set_name(&name);
    }

    session.create(material);
    Ok(NonError::Continue)
}

pub async fn create_material_non_interactive(
    arg_matches: &ArgMatches,
    session: &mut Session,
) -> Result<NonError, Error> {
    let mut material = MaterialItem::new();

    if let Some(name) = arg_matches.get_one::<String>("name") {
        material.set_name(name);
    }

    session.create(material);
    Ok(NonError::Continue)
}
