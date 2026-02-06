use business_planner::api::{graphs::{ProductionLine, Recipe}, session::Session};
use clap::Command;
use enum_map::Enum;
use inquire::Select;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumString};

use crate::{Error, NonError, graphs::GraphsType, utils::{Menu, PlannerResult, prompt_user_select_graph}};

#[derive(Debug, Display, Enum, EnumString)]
pub enum ReadGraphMenu {}

impl Menu for ReadGraphMenu {
    fn get_command() -> Command {
        Command::new("read")
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
                let graph_id = prompt_user_select_graph::<ProductionLine>(session, "Select graph").await?;
                let graph_data = session.read_graph::<ProductionLine>(graph_id);
                println!("{:?}", graph_data);
            },
            GraphsType::Recipe => {
                let graph_id = prompt_user_select_graph::<Recipe>(session, "Select graph").await?;
                let graph_data = session.read_graph::<Recipe>(graph_id);
                println!("{:?}", graph_data);
            }
        }
        Ok(NonError::Continue)
    }
}
