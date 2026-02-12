use std::{cmp::Ordering, str::FromStr};

use business_planner::api::{graphs::{ProductionLine, Recipe}, item::{IngredientItem, StoreItem}, session::Session};
use clap::Command;
use inquire::Select;
use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::{Error, NonError, utils::{Menu, NodeItemTypes, PlannerResult, prompt_select_command, prompt_user_select_graph}};

pub struct RemoveMenu {}

impl Menu for RemoveMenu {
    type SubcommandsEnum = NodeItemTypes;

    fn get_command() -> Command {
        Command::new("remove")
    }

    async fn interactive(session: &mut Session) -> PlannerResult {
        let subcommands = Self::SubcommandsEnum::iter();
        let selected_subcommand = prompt_select_command(subcommands)?;
        
        match selected_subcommand {
            Self::SubcommandsEnum::Store => {
                let production_line_id = prompt_user_select_graph::<ProductionLine>(session, "Select production line to remove store from").await?.to_owned();
                
                let store_ids= session.list_nodes::<ProductionLine>(&production_line_id)?;
                let store_names = store_ids.iter().map(|store_id| {
                    session.read::<StoreItem>(store_id).expect("store ids returned to be valid").get_name()
                }).collect::<Vec<_>>();
                let selected_option =  Select::new("Select store to remove", store_names).raw_prompt_skippable()?.ok_or(Error::UserCancelled)?;
                let selected_store_id = store_ids.get(selected_option.index).expect("");
                
                session.remove_node::<ProductionLine>(&production_line_id, selected_store_id)?;
            },
            Self::SubcommandsEnum::Ingredient => {
                let recipe_id = prompt_user_select_graph::<Recipe>(session, "Select recipe to remove ingredient from").await?.to_owned();
                
                let ingredient_ids= session.list_nodes::<Recipe>(&recipe_id)?;
                let ingredient_names = ingredient_ids.iter().map(|ingredient_id| {
                    session.read::<IngredientItem>(ingredient_id).expect("ingredient ids returned to be valid").get_name()
                }).collect::<Vec<_>>();
                let selected_option =  Select::new("Select ingredient to remove", ingredient_names).raw_prompt_skippable()?.ok_or(Error::UserCancelled)?;
                let selected_ingredient_id = ingredient_ids.get(selected_option.index).expect("");
                
                session.remove_node::<Recipe>(&recipe_id, selected_ingredient_id)?;
            }
        };
        Ok(NonError::Continue)
    }

    async fn non_interactive(arg_matches: &clap::ArgMatches, session: &mut Session) -> PlannerResult {
        let (item_type, subcommand_arg_matches) = arg_matches.subcommand().expect("Clap to have filtered off invalid input");
        let item_type = Self::SubcommandsEnum::from_str(item_type)?;

        match item_type {
            Self::SubcommandsEnum::Store => {
                let store_id = match subcommand_arg_matches.get_one::<Uuid>("store_id") {
                    Some(store_id) => store_id,
                    None => {
                        let store_name = subcommand_arg_matches.get_one::<String>("store_name").expect("");
                        let mut store_ids = session.get_by_name::<StoreItem>(store_name);
                        match (store_ids.len()).cmp(&1) {
                            Ordering::Less => Err(Error::NotFound(format!("store with the name {}", store_name)))?,
                            Ordering::Equal => store_ids.remove(0),
                            Ordering::Greater => Err(Error::MultipleFound(format!("store with the name {}", store_name)))?,
                        }
                    }
                }.to_owned();
                
                let production_line_id = match subcommand_arg_matches.get_one::<Uuid>("production_line_id") {
                    Some(production_line_id) => production_line_id,
                    None => {
                        let production_line_name = subcommand_arg_matches.get_one::<String>("store_name").expect("");
                        let mut production_line_ids = session.get_graphs_by_name::<ProductionLine>(production_line_name);
                        match (production_line_ids.len()).cmp(&1) {
                            Ordering::Less => Err(Error::NotFound(format!("production line with the name {}", production_line_name)))?,
                            Ordering::Equal => production_line_ids.remove(0),
                            Ordering::Greater => Err(Error::MultipleFound(format!("production line with the name {}", production_line_name)))?,
                        }
                    }
                }.to_owned();

                session.remove_node::<ProductionLine>(&store_id, &production_line_id)?;
            },
            Self::SubcommandsEnum::Ingredient => {
                let ingredient_id = match subcommand_arg_matches.get_one::<Uuid>("ingredient_id") {
                    Some(ingredient_id) => ingredient_id,
                    None => {
                        let ingredient_name = subcommand_arg_matches.get_one::<String>("ingredient_name").expect("");
                        let mut ingredient_ids = session.get_by_name::<IngredientItem>(ingredient_name);
                        match (ingredient_ids.len()).cmp(&1) {
                            Ordering::Less => Err(Error::NotFound(format!("ingredient with the name {}", ingredient_name)))?,
                            Ordering::Equal => ingredient_ids.remove(0),
                            Ordering::Greater => Err(Error::MultipleFound(format!("ingredient with the name {}", ingredient_name)))?,
                        }
                    }
                }.to_owned();
                
                let recipe_id = match subcommand_arg_matches.get_one::<Uuid>("recipe_id") {
                    Some(recipe_id) => recipe_id,
                    None => {
                        let recipe_name = subcommand_arg_matches.get_one::<String>("recipe_name").expect("");
                        let mut recipe_ids = session.get_graphs_by_name::<Recipe>(recipe_name);
                        match (recipe_ids.len()).cmp(&1) {
                            Ordering::Less => Err(Error::NotFound(format!("recipe with the name {}", recipe_name)))?,
                            Ordering::Equal => recipe_ids.remove(0),
                            Ordering::Greater => Err(Error::MultipleFound(format!("recipe with the name {}", recipe_name)))?,
                        }
                    }
                }.to_owned();

                session.remove_node::<Recipe>(&ingredient_id, &recipe_id)?;
            },
        };

        Ok(NonError::Continue)
    }
}