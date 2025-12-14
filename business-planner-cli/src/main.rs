use std::path::PathBuf;

use business_planner::api::session::{Session, create_session, load_session};
use clap::{Arg, ArgAction, ArgMatches, Command};

pub mod error;
pub mod shells;
pub mod registry;
pub mod save;
pub mod plugins;
pub mod utils;

pub use error::{NonError, Error};

use crate::{plugins::{get_plugins_subcommand, parse_interactive_plugins_subcommand}, registry::{get_registry_subcommand, parse_interactive_registry_subcommand, parse_non_interactive_registry_subcommand}, save::{get_save_interactive_subcommand, get_save_subcommand, parse_interactive_save_subcommand, parse_non_interactive_save_subcommand}, shells::{interactive, non_interactive}};

fn entry_cli() -> Command {
    Command::new("business-planner-cli")
        .arg(
            Arg::new("interactive")
                .global(true)
                .required(false)
                .long("interactive")
                .action(ArgAction::SetTrue)
        )
        .subcommands([
            Command::new("create"),
            Command::new("load")
                .arg(
                    Arg::new("path")
                        .required(true)
                        .value_parser(clap::value_parser!(PathBuf))
                ),
        ])
}

#[tokio::main]
async fn main () {
    let matches = entry_cli().get_matches();
    let mut session = match matches.subcommand() {
        Some(("load", arg_matches)) => {
            let path = arg_matches.get_one::<PathBuf>("path").expect("Clap to have filtered off missing path argument");
            load_session(path).unwrap()
        },
        _ => {
            create_session()
        }
    };
    let interactive = matches.get_one::<bool>("interactive").unwrap_or(&false);
    let mut user_requested_exit = false;
    while !user_requested_exit {
        let result = match interactive {
            true => interactive::shell(
                get_main_menu_subcommand().get_subcommands().map(|command| {
                    command.get_name().to_string()
                }).collect(),
                parse_interactive_main_menu_subcommand,
                &mut session
            ).await,
            false => non_interactive::shell(
                get_main_menu_subcommand(),
                parse_non_interactive_main_menu_subcommand,
                &mut session
            ).await,
        };
        
        match result {
            Ok(NonError::Exit) => { user_requested_exit = true; },
            Ok(NonError::Continue) => {},
            Err(Error::UserCancelled) => {},
            Err(error) => println!("{:?}", error),
        };
    }
}

fn get_main_menu_subcommand() -> Command {
    Command::new("")
        .no_binary_name(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands([
            get_plugins_subcommand(),
            get_registry_subcommand(),
            get_save_subcommand(),
            Command::new("exit"),
        ])
}

async fn parse_interactive_main_menu_subcommand(command: &str, session: &mut Session) -> Result<NonError, Error> {
    match command {
        "plugins" => {
            shells::interactive::shell(
                get_plugins_subcommand().get_subcommands().map(|command| {
                    command.get_name().to_string()
                }).collect(),
                parse_interactive_plugins_subcommand,
                session,
            ).await
        },
        "registry" => {
            shells::interactive::shell(
                get_registry_subcommand().get_subcommands().map(|command| {
                    command.get_name().to_string()
                }).collect(),
                parse_interactive_registry_subcommand,
                session,
            ).await
        },
        "save" => {
            shells::interactive::shell(
                get_save_interactive_subcommand(),
                parse_interactive_save_subcommand,
                session,
            ).await
        },
        "exit" => Ok(NonError::Exit),
        _ => Err(Error::InvalidInput),
    }
}

async fn parse_non_interactive_main_menu_subcommand(arg_matches: &ArgMatches, session: &mut Session) -> Result<NonError, Error> {
    match arg_matches.subcommand() {
        Some(("registry", arg_matches)) => {
            parse_non_interactive_registry_subcommand(arg_matches, session).await
        },
        Some(("save", arg_matches)) => {
            parse_non_interactive_save_subcommand(arg_matches, session).await
        },
        Some(("exit", _)) => Ok(NonError::Exit),
        _ => Err(Error::InvalidInput),
    }
}