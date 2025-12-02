use business_planner::api::{registry::Material, session::Session};
use clap::{Arg, ArgMatches, Command};
use inquire::Text;

use crate::{Error, NonError};

pub fn get_create_material_subcommand() -> Command {
    Command::new("material")
        .no_binary_name(true)
        .arg(
            Arg::new("name")
                .long("name")
        )
}

pub async fn create_material_interactive_subcommand(session: &mut Session) -> Result<NonError, Error> {
    let mut material = Material::new();

    if let Some(name) = Text::new("name")
        .with_help_message("Material name.")
        .prompt_skippable()?
    {
        material.set_name(&name);
    }

    session.create(material);
    Ok(NonError::Continue)
}

pub async fn parse_create_material_non_interactive_subcommand(arg_matches: &ArgMatches, session: &mut Session) -> Result<NonError, Error> {
    let mut material = Material::new();
    
    if let Some(name) = arg_matches.get_one::<String>("name") {
        material.set_name(name);
    }

    session.create(material);
    Ok(NonError::Continue)
}
