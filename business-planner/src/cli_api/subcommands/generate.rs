use std::fs;

use crate::structs::Session;
use crate::cli_api::error::Error;
use clap::Subcommand;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;

// #[derive(Debug, Subcommand, EnumIter)]
// pub enum Command {
    
// }

pub enum SearchPluginError {
    Error
}

pub fn get_commands () -> Result<Vec<String>, SearchPluginError> {
    let dir_entries = fs::read_dir("./plugins/").unwrap();
    dir_entries.map(|dir_entry| {
        let Ok(path) = dir_entry else {
            return Err(SearchPluginError::Error)
        };
        match path.file_name().into_string() {
            Ok(filename) => Ok(filename),
            _ => Err(SearchPluginError::Error)
        }
    }).collect::<Result<Vec<String>, _>>()
}

// pub fn parse_interactive_subcommand (command: &str, session: &Session, _user_requested_exit: &mut bool) -> Result<(), Error> {

// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        if let Ok(result) = get_commands() {
            println!("{:#?}", result);
        }
    }
}
