use std::{path::PathBuf, str::FromStr};

use business_planner::api::session::{Session, save_to_last_save_location, save_to_location};
use clap::{Arg, ArgMatches, Command, value_parser};
use inquire::Text;

use crate::{Error, NonError};

pub fn get_save_subcommand() -> Command {
    Command::new("save")
        .no_binary_name(true)
        .arg(
            Arg::new("path")
                .required(false)
                .value_parser(value_parser!(PathBuf))
        )
}

pub fn get_save_interactive_subcommand() -> Vec<String> {
    vec!["save".to_string(), "save as".to_string()]
}

pub async fn parse_interactive_save_subcommand(command: &str, session: &mut Session) -> Result<NonError, Error> {
    match command {
        "save" => {
            save_to_last_save_location(session, true)?;
            Ok(NonError::Continue)
        },
        "save as" => {
            let Ok(path) = Text::new("path").prompt() else {
                return Err(Error::UserCancelled)
            };
            let path = PathBuf::from_str(&path).expect("Pathbuf from String to be infallible");
            save_to_location(session, &path, true)?;
            Ok(NonError::Continue)
        },
        _ => Err(Error::InvalidInput),
    }
}

pub async fn parse_non_interactive_save_subcommand(arg_matches: &ArgMatches, session: &mut Session) -> Result<NonError, Error> {
    let path = arg_matches.get_one::<PathBuf>("path");

    match path {
        Some(path) => {
            save_to_location(session, path, true)?;
            Ok(NonError::Continue)
        },
        None => {
            save_to_last_save_location(session, true)?;
            Ok(NonError::Continue)
        },
    }
}