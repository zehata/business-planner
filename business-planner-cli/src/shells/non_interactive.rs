use std::io::{Write, stdin, stdout};
use business_planner::api::session::Session;
use clap::{Parser};
use crate::{error, subcommands::top_level::Command};

#[derive(Debug, Parser)]
#[command(name = "")]
#[command(about, no_binary_name(true))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

pub fn prompt_user<T>(
    parser: T,
    session: &Session,
    user_requested_exit: &mut bool
) -> Result<(), error::Error>
where
T: FnOnce(&Command, &Session, &mut bool) -> Result<(), error::Error>
{
    print!("> ");
    stdout().flush().expect("Failed to print to stdout");

    let buffer = prompt_user_text()?;
    
    let cli_result = Cli::try_parse_from(buffer.split_whitespace())?;
    parser(&cli_result.command, session, user_requested_exit)
}

pub fn prompt_user_text () -> Result<String, error::Error> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;

    Ok(buffer)
}