use business_planner::api::{graphs::{Graph, ProductionLine, Recipe}, session::Session};
use clap::Command;
use enum_map::Enum;
use inquire::Select;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumString};

use crate::{Error, NonError, graphs::GraphsType, utils::{Menu, PlannerResult, print_list}};

#[derive(Debug, Display, Enum, EnumString)]
pub enum ListGraphsMenu {}

impl Menu for ListGraphsMenu {
    fn get_command() -> Command {
        Command::new("list")
    }

    async fn interactive(session: &mut Session) -> PlannerResult {
        let commands = GraphsType::iter().collect::<Vec<_>>();
        let command_names = commands
            .iter()
            .map(|command| format!("{}", command))
            .collect();
        let selected_command = Select::new("Select", command_names)
            .raw_prompt_skippable()?
            .ok_or(Error::UserCancelled)?;
        let command = commands
            .get(selected_command.index)
            .expect("User cannot select if subcommands is empty");

        let graphs = match command {
            GraphsType::ProductionLine => {
                session.list_graphs::<ProductionLine>().map(|(_, graph)| {
                    graph.get_name()
                }).collect::<Vec<_>>()
            },
            GraphsType::Recipe => {
                session.list_graphs::<Recipe>().map(|(_, graph)| {
                    graph.get_name()
                }).collect::<Vec<_>>()
            }
        };
        print_list(graphs);
        Ok(NonError::Continue)
    }
}
