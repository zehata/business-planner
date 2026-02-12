use business_planner::api::{graphs::Recipe, item::IngredientItem, session::Session};
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

        let id = session.create(ingredient);
        println!("Created ingredient \"{}\" ({})", name, id);
        Ok(NonError::Continue)
    }

    async fn non_interactive(arg_matches: &ArgMatches, session: &mut Session) -> PlannerResult {
        let mut ingredient = IngredientItem::new();

        let ingredient_name = arg_matches.get_one::<String>("ingredient_name").expect("Clap to have filtered off invalid input");
        ingredient.set_name(ingredient_name);


        let recipe_id = RecipeArg::parse_arg_matches(arg_matches, session)?;

        let ingredient_id = session.create(ingredient);    
        println!("Created ingredient \"{}\" ({})", ingredient_name, ingredient_id);

        if
            let Some(recipe_id) = recipe_id
            && session.add_node::<Recipe>(&ingredient_id, &recipe_id).is_ok()
        {
            let recipe_name = session.read_graph::<Recipe>(&recipe_id).expect("Graph to exist").get_name();
            println!("Added ingredient \"{}\" to recipe \"{}\"", ingredient_name, recipe_name);
        };

        Ok(NonError::Continue)
    }
}
