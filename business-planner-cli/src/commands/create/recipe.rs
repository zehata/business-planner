use business_planner::api::{graphs::Recipe, session::Session};
use clap::Command;

use crate::{NonError, utils::{Menu, NoSubcommands, PlannerResult, RecipeArg}};

pub struct CreateRecipeMenu {}

impl Menu for CreateRecipeMenu {
    type SubcommandsEnum = NoSubcommands;

    fn get_command() -> clap::Command {
        Command::new("recipe")
            .args(RecipeArg::with_alias())
    }

    async fn interactive(session: &mut Session) -> PlannerResult {
        session.create_graph::<Recipe>("");
        Ok(NonError::Continue)
    }

    async fn non_interactive(arg_matches: &clap::ArgMatches, session: &mut Session) -> PlannerResult {
        let name = match arg_matches.get_one::<String>("recipe_name") {
            Some(name) => name,
            None => "",
        };
        session.create_graph::<Recipe>(name);
        Ok(NonError::Continue)
    }
}