use std::{fmt, path::PathBuf};
use business_planner::session::Session;
use clap::Subcommand;
use strum_macros::EnumIter;

use crate::{errors::{Error, ParseError}, subcommands::save::save};

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
                Some(path) => session.save_to_location(path),
                None => session.save_to_last_save_location(),
            }?;
        },
        Command::Exit => {
            *user_requested_exit = true;
        },
    };
    Ok(())
}

pub fn parse_interactive_subcommand(command: &str, session: &Session, user_requested_exit: &mut bool) -> Result<(), Error> {
    match command {
        "Save" => {
            save(session)
        },
        "Exit" => {
            *user_requested_exit = true;
            Ok(())
        },
        _ => Err(Error::ParseError(ParseError::InvalidCommandError(command.to_string()))),
    }
}