use business_planner::api::{item::MaterialItem, session::Session};
use clap::{ArgMatches, Command};
use inquire::Text;

use crate::{
    NonError,
    utils::{MaterialArg, Menu, NoSubcommands, PlannerResult},
};

pub struct CreateMaterialMenu {}

impl Menu for CreateMaterialMenu {
    type SubcommandsEnum = NoSubcommands;

    fn get_command() -> Command {
        Command::new("material")
            .args(MaterialArg::with_alias())
    }

    async fn interactive(session: &mut Session) -> PlannerResult {
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

    async fn non_interactive(arg_matches: &ArgMatches, session: &mut Session) -> PlannerResult {
        let mut material = MaterialItem::new();

        if let Some(name) = arg_matches.get_one::<String>("material_name") {
            material.set_name(name);
        }

        session.create(material);
        Ok(NonError::Continue)
    }
}