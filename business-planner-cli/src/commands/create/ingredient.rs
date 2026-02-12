use business_planner::api::{item::IngredientItem, session::Session};
use clap::{ArgMatches, Command};
use inquire::Text;

use crate::{
    Error, NonError,
    utils::{IngredientArg, Menu, NoSubcommands, PlannerResult, RecipeArg},
};

pub struct CreateIngredientMenu {}

impl Menu for CreateIngredientMenu {
    type SubcommandsEnum = NoSubcommands;

    fn get_command() -> Command {
        Command::new("ingredient")
            .args(IngredientArg::with_alias())
            .args(RecipeArg::optional())
    }

    async fn interactive(session: &mut Session) -> PlannerResult {
        let mut ingredient = IngredientItem::new();

        let name = Text::new("name")
            .with_help_message("ingredient name")
            .prompt_skippable()?;
        let Some(name) = name else {
            return Err(Error::UserCancelled);
        };
        ingredient.set_name(&name);

        session.create(ingredient);
        Ok(NonError::Continue)
    }

    async fn non_interactive(arg_matches: &ArgMatches, session: &mut Session) -> PlannerResult {
        let mut ingredient = IngredientItem::new();

        if let Some(name) = arg_matches.get_one::<String>("ingredient_name") {
            ingredient.set_name(name);
        }

        session.create(ingredient);
        Ok(NonError::Continue)
    }
}
