use std::io::{Write, stdin, stdout};

use business_planner::api::session::Session;
use clap::{ArgMatches, Command};

use crate::{Error, NonError};

pub async fn shell<T>(
    command: Command,
    parse: T,
    session: &mut Session,
) -> Result<NonError, Error> 
where
T: AsyncFnOnce(&ArgMatches, &mut Session) -> Result<NonError, Error>,
{
    print!("> ");
    stdout().flush().expect("Failed to print to stdout");

    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("Error reading stdin");
    
    let arg_matches = command.try_get_matches_from(buffer.split_whitespace())?;

    parse(&arg_matches, session).await
}