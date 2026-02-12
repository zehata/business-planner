use business_planner::api::{graphs::ProductionLine, session::Session};
use clap::Command;

use crate::{NonError, utils::{Menu, NoSubcommands, PlannerResult, ProductionLineArg}};

pub struct CreateProductionLineMenu {}

impl Menu for CreateProductionLineMenu {
    type SubcommandsEnum = NoSubcommands;

    fn get_command() -> clap::Command {
        Command::new("production-line")
            .visible_alias("line")
            .args(ProductionLineArg::with_alias())
    }

    async fn interactive(session: &mut Session) -> PlannerResult {
        session.create_graph::<ProductionLine>("");
        Ok(NonError::Continue)
    }

    async fn non_interactive(arg_matches: &clap::ArgMatches, session: &mut Session) -> PlannerResult {
        let name = match arg_matches.get_one::<String>("production_line_name") {
            Some(name) => name,
            None => "",
        };
        session.create_graph::<ProductionLine>(name);
        Ok(NonError::Continue)
    }
}