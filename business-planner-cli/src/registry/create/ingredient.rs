use business_planner::api::{item::IngredientItem, session::Session};
use clap::{Arg, ArgMatches, Command};
use enum_map::Enum;
use inquire::Text;
use strum_macros::{Display, EnumString};

use crate::{
    Error, NonError,
    utils::{Menu, PlannerResult},
};

#[derive(Debug, Display, Enum, EnumString)]
pub enum CreateIngredientMenu {}

impl Menu for CreateIngredientMenu {
    fn get_command() -> Command {
        Command::new("ingredient")
            .no_binary_name(true)
            .arg(Arg::new("name").long("name"))
            .arg(Arg::new("timestamps_source").long("timestamps_source"))
            .arg(
                Arg::new("timestamps_file")
                    .long("timestamps_file")
                    .required_if_eq_any([
                        ("timestamps_source", "excel"),
                        ("timestamps_source", "csv"),
                    ])
                    .requires("timestamps_range")
                    .conflicts_with("timestamps_query"),
            )
            .arg(
                Arg::new("timestamps_range")
                    .long("timestamps_range")
                    .required_if_eq_any([
                        ("timestamps_source", "excel"),
                        ("timestamps_source", "csv"),
                    ])
                    .requires("timestamps_file")
                    .conflicts_with("timestamps_query"),
            )
            .arg(
                Arg::new("timestamps_query")
                    .long("timestamps_query")
                    .required_if_eq("timestamps_source", "psql")
                    .conflicts_with_all(["timestamps_file", "timestamps_range"]),
            )
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

        if let Some(name) = arg_matches.get_one::<String>("name") {
            ingredient.set_name(name);
        }

        session.create(ingredient);
        Ok(NonError::Continue)
    }
}
