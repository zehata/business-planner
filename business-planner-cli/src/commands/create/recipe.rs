use business_planner::api::{graphs::Recipe, session::Session};
use clap::Command;
use inquire::Text;

use crate::{Error, NonError, utils::{Menu, NoSubcommands, PlannerResult, RecipeArg}};

pub struct CreateRecipeMenu {}

impl Menu for CreateRecipeMenu {
    type SubcommandsEnum = NoSubcommands;

    fn get_command() -> clap::Command {
        Command::new("recipe")
            .args(RecipeArg::with_alias())
    }

    async fn interactive(session: &mut Session) -> PlannerResult {
        let name = Text::new("name")
            .with_help_message("Recipe name")
            .prompt_skippable()?;
        let Some(name) = name else {
            return Err(Error::UserCancelled);
        };

        let id = session.create_graph::<Recipe>(&name);
        println!("Created recipe \"{}\" ({})", name, id);
        Ok(NonError::Continue)
    }

    async fn non_interactive(arg_matches: &clap::ArgMatches, session: &mut Session) -> PlannerResult {
        let name = arg_matches.get_one::<String>("recipe_name").expect("Clap to have filtered off invalid input");

        let id = session.create_graph::<Recipe>(name);
        println!("Created recipe \"{}\" ({})", name, id);
        Ok(NonError::Continue)
    }
}