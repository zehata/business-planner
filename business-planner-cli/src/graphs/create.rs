use business_planner::api::{graphs::{ProductionLine, Recipe}, session::Session};
use clap::Command;
use enum_map::Enum;
use inquire::Select;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumString};

use crate::{Error, NonError, graphs::GraphsType, utils::{Menu, PlannerResult}};

#[derive(Debug, Display, Enum, EnumString)]
pub enum CreateGraphMenu {}

impl Menu for CreateGraphMenu {
    fn get_command() -> Command {
        Command::new("create")
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
        match command {
            GraphsType::ProductionLine => {
                session.create_graph::<ProductionLine>();
            },
            GraphsType::Recipe => {
                session.create_graph::<Recipe>();
            }
        }
        Ok(NonError::Continue)
    }
}
