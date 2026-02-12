use business_planner::api::{item::MaterialItem, session::Session};
use clap::{ArgMatches, Command};
use inquire::Text;

use crate::{
    Error, NonError, utils::{MaterialArg, Menu, NoSubcommands, PlannerResult}
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

        let name = Text::new("name")
            .with_help_message("Material name")
            .prompt_skippable()?;
        let Some(name) = name else {
            return Err(Error::UserCancelled);
        };
        material.set_name(&name);

        let id = session.create(material);
        println!("Created material \"{}\" ({})", name, id);
        Ok(NonError::Continue)
    }

    async fn non_interactive(arg_matches: &ArgMatches, session: &mut Session) -> PlannerResult {
        let mut material = MaterialItem::new();

        let name = arg_matches.get_one::<String>("material_name").expect("Clap to have filtered off invalid input");
        material.set_name(name);

        let id = session.create(material);
        println!("Created material \"{}\" ({})", name, id);
        Ok(NonError::Continue)
    }
}