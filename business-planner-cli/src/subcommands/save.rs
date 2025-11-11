use std::{fmt, path::{Path, PathBuf}};
use business_planner::session::Session;
use inquire::Text;
use clap::Subcommand;
use strum_macros::EnumIter;

use crate::errors::{Error, ParseError};

#[derive(Debug, Subcommand, EnumIter)]
pub enum Command {
    /// Saves the current planner file
    #[command()]
    SaveAs {
        /// Path to save file to
        #[arg()]
        path: Option<PathBuf>,
    },
    /// Saves the current planner file
    #[command()]
    Save,   
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Self::SaveAs{path: _} => "Save as",
            Self::Save => "Save",
        };
        write!(f, "{}", name)
    }
}

pub fn parse_interactive_command(command: &str, session: &Session, _user_requested_exit: &mut bool) -> Result<(), Error> {
    match command {
        "Save" => {
            Ok(session.save_to_last_save_location(true)?)
        },
        "Save as" => {
            let default_path = match &session.last_save_location {
                Some(path) => { path.to_str().unwrap_or("") },
                _ => "",
            };
            let ans = Text::new("Save to: ").with_default(default_path).prompt()?;
            let path = Path::new(&ans);
            Ok(session.save_to_location(path, false)?)
        },
        _ => Err(Error::ParseError(ParseError::InvalidCommandError(command.to_string())))?,
    }
}