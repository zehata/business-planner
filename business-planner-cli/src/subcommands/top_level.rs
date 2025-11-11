use std::{fmt, path::PathBuf};
use business_planner::session::Session;
use clap::Subcommand;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;

use crate::{errors::{Error, ParseError}, shells::{self}, subcommands::{self}};

#[derive(Debug, Subcommand, EnumIter)]
pub enum Command {
    /// Saves the current planner file
    #[command()]
    Save {
        /// Path to save file to
        #[arg()]
        path: Option<PathBuf>,
    },
    /// Saves the current planner file
    #[command()]
    Exit,   
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Self::Save{path: _} => "Save",
            Self::Exit => "Exit",
        };
        write!(f, "{}", name)
    }
}

pub fn parse_non_interactive_subcommand (command: &Command, session: &Session, user_requested_exit: &mut bool) -> Result<(), Error> {
    match command {
        Command::Save { path } => {
            match path {
                Some(path) => session.save_to_location(path, false),
                None => session.save_to_last_save_location(true),
            }?;
        },
        Command::Exit => {
            *user_requested_exit = true;
        },
    };
    Ok(())
}

pub fn parse_interactive_subcommand(command: &str, session: &Session, user_requested_exit: &mut bool) -> Result<(), Error> {
    let commands: Vec<_> = subcommands::save::Command::iter().clone().map(|command| { format!("{}", command) }).collect();
    let commands = commands.iter().map(|command| { command.as_str() }).collect();

    match command {
        "Save" => shells::interactive::prompt_user(
            commands,
            subcommands::save::parse_interactive_command,
            session,
            user_requested_exit
        ),
        "Exit" => {
            *user_requested_exit = true;
            Ok(())
        },
        _ => Err(Error::ParseError(ParseError::InvalidCommandError(command.to_string()))),
    }
}