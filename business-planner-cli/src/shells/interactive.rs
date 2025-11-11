use business_planner::session::Session;
use inquire::Select;
use crate::{errors::Error, subcommands::top_level::CommandIter};

pub fn prompt_user<T>(
    commands: CommandIter,
    parser: T,
    session: &Session,
    user_requested_exit: &mut bool
) -> Result<(), Error>
where T: FnOnce(&str, &Session, &mut bool) -> Result<(), Error>
{
    let commands: Vec<_> = commands.clone().map(|command| { format!("{}", command) }).collect();
    let commands = commands.iter().map(|command| { command.as_str() }).collect();

    let ans = Select::new("Select", commands).prompt()?;

    parser(ans, session, user_requested_exit)
}