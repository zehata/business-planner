use std::{fmt, path::{Path, PathBuf}};
use business_planner::api::session::{Session, save_to_last_save_location, save_to_location};
use inquire::Text;
use clap::Subcommand;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;

use crate::error;

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

pub fn get_commands () -> Vec<String> {
    Command::iter().clone().map(|command| { format!("{}", command) }).collect()
}

pub fn parse_interactive_command(command: &str, session: &Session, _user_requested_exit: &mut bool) -> Result<(), error::Error> {
    match command {
        "Save" => {
            Ok(save_to_last_save_location(session, true)?)
        },
        "Save as" => {
            let default_path = match &session.last_save_location {
                Some(path) => { path.to_str().unwrap_or("") },
                _ => "",
            };
            let ans = Text::new("Save to: ").with_default(default_path).prompt()?;
            let path = Path::new(&ans);
            Ok(save_to_location(session, path, false)?)
        },
        _ => Err(error::Error::ParseError(error::ParseError::InvalidCommandError(command.to_string())))?,
    }
}