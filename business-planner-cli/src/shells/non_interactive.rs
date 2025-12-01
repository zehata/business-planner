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
    
    parse_buffer(&buffer, command, parse, session).await
}


pub async fn parse_buffer<T>(
    buffer: &str,
    command: Command,
    parse: T,
    session: &mut Session,
) -> Result<NonError, Error> 
where
T: AsyncFnOnce(&ArgMatches, &mut Session) -> Result<NonError, Error>,
{
    let arg_matches = get_command_matches(buffer, command)?;
    parse(&arg_matches, session).await
}

pub fn get_command_matches(buffer: &str, command: Command) -> Result<ArgMatches, Error> {
    let Some(args) = shlex::split(buffer) else {
        return Err(Error::ErroneousShlexInput)
    };
    let arg_matches = command.try_get_matches_from(args)?;
    Ok(arg_matches)
}