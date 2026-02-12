use std::str::FromStr;

use business_planner::api::{graphs::{ProductionLine, Recipe}, item::{IngredientItem, StoreItem}, session::Session};
use clap::Command;
use strum::IntoEnumIterator;

use crate::{NonError, utils::{IngredientArg, Menu, NodeItemTypes, PlannerResult, ProductionLineArg, RecipeArg, StoreArg, prompt_select_command, prompt_user_select_graph, prompt_user_select_registry_item}};

pub struct AddMenu {}

impl Menu for AddMenu {
    type SubcommandsEnum = NodeItemTypes;

    fn get_command() -> Command {
        Command::new("add")
            .subcommands(Self::get_subcommands())
    }

    fn get_subcommands() -> impl Iterator<Item = Command> {
        [
            Command::new("store")
                .args(StoreArg::with_alias())
                .args(ProductionLineArg::without_alias()),
            Command::new("ingredient")
                .args(IngredientArg::with_alias())
                .args(RecipeArg::without_alias()),
        ].into_iter()
    }

    async fn interactive(session: &mut Session) -> PlannerResult {
        let subcommands = Self::SubcommandsEnum::iter();
        let selected_subcommand = prompt_select_command(subcommands)?;
        
        match selected_subcommand {
            Self::SubcommandsEnum::Store => {
                let production_line_id = prompt_user_select_graph::<ProductionLine>(session, "Select production line to add store to").await?.to_owned();
                let store_id = prompt_user_select_registry_item::<StoreItem>(session, "Select store to add").await?.to_owned();
                session.add_node::<ProductionLine>(&store_id, &production_line_id)?;
            },
            Self::SubcommandsEnum::Ingredient => {
                let production_line_id = prompt_user_select_graph::<Recipe>(session, "Select recipe to add ingredient to").await?.to_owned();
                let store_id = prompt_user_select_registry_item::<IngredientItem>(session, "Select ingredient to add").await?.to_owned();
                session.add_node::<Recipe>(&store_id, &production_line_id)?;
            }
        };
        Ok(NonError::Continue)
    }

    async fn non_interactive(arg_matches: &clap::ArgMatches, session: &mut Session) -> PlannerResult {
        let (item_type, subcommand_arg_matches) = arg_matches.subcommand().expect("Clap to have filtered off invalid input");
        let item_type = Self::SubcommandsEnum::from_str(item_type)?;

        match item_type {
            Self::SubcommandsEnum::Store => {
                let store_id = StoreArg::parse_arg_matches(subcommand_arg_matches, session)?;
                let production_line_id = ProductionLineArg::parse_arg_matches(subcommand_arg_matches, session)?;

                session.add_node::<ProductionLine>(&store_id, &production_line_id)?;
            },
            Self::SubcommandsEnum::Ingredient => {
                let ingredient_id = IngredientArg::parse_arg_matches(subcommand_arg_matches, session)?;
                let recipe_id = RecipeArg::parse_arg_matches(subcommand_arg_matches, session)?;

                session.add_node::<Recipe>(&ingredient_id, &recipe_id)?;
            },
        };

        Ok(NonError::Continue)
    }
}