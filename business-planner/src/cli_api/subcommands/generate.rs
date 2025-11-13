use crate::structs::Session;
use crate::cli_api::error::Error;
use clap::Subcommand;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;

#[derive(Debug, Subcommand, EnumIter)]
pub enum Command {
    
}

pub fn get_commands () -> Vec<String> {
    Command::iter().clone().map(|command| { format!("{}", command) }).collect()
}

pub fn parse_interactive_subcommand (command: &str, session: &Session, _user_requested_exit: &mut bool) -> Result<(), Error> {

}