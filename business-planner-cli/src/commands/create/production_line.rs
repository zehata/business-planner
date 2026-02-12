use business_planner::api::{graphs::ProductionLine, session::Session};
use clap::Command;
use inquire::Text;

use crate::{Error, NonError, utils::{Menu, NoSubcommands, PlannerResult, ProductionLineArg}};

pub struct CreateProductionLineMenu {}

impl Menu for CreateProductionLineMenu {
    type SubcommandsEnum = NoSubcommands;

    fn get_command() -> clap::Command {
        Command::new("production-line")
            .visible_alias("line")
            .args(ProductionLineArg::with_alias())
    }

    async fn interactive(session: &mut Session) -> PlannerResult {
        let name = Text::new("Production line name")
            .with_help_message("Production line name")
            .prompt_skippable()?;
        let Some(name) = name else {
            return Err(Error::UserCancelled);
        };

        let id = session.create_graph::<ProductionLine>(&name);
        println!("Created production line \"{}\" ({})", name, id);
        Ok(NonError::Continue)
    }

    async fn non_interactive(arg_matches: &clap::ArgMatches, session: &mut Session) -> PlannerResult {
        let name = arg_matches.get_one::<String>("production_line_name").expect("Clap to have filtered off invalid input");

        let id = session.create_graph::<ProductionLine>(name);
        println!("Created production line \"{}\" ({})", name, id);
        Ok(NonError::Continue)
    }
}