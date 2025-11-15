use std::{fmt, path::PathBuf};
use business_planner::api::{session::{Session, save_to_last_save_location, save_to_location}};
use clap::Subcommand;
use strum_macros::EnumIter;

use crate::{shells, subcommands, error};

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

pub fn parse_non_interactive_subcommand (command: &Command, session: &Session, user_requested_exit: &mut bool) -> Result<(), error::Error> {
    match command {
        Command::Save { path } => {
            match path {
                Some(path) => save_to_location(session, path, false),
                None => save_to_last_save_location(session, true),
            }?;
        },
        Command::Exit => {
            *user_requested_exit = true;
        },
    };
    Ok(())
}

pub fn parse_interactive_subcommand(command: &str, session: &Session, user_requested_exit: &mut bool) -> Result<(), error::Error> {
    match command {
        "Save" => shells::interactive::prompt_user(
            || { Ok(subcommands::save::get_commands()) },
            subcommands::save::parse_interactive_command,
            session,
            user_requested_exit,
        ),
        "Generate" => shells::interactive::prompt_user(
            || { Ok(subcommands::generate::get_commands()?) },
            subcommands::generate::parse_interactive_command,
            session,
            user_requested_exit,
        ),
        "Exit" => {
            *user_requested_exit = true;
            Ok(())
        },
        _ => Err(error::Error::ParseError(error::ParseError::InvalidCommandError(command.to_string()))),
    }
}