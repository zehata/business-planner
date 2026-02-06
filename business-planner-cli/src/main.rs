use std::{io::{Write, stdin, stdout}, path::PathBuf};

use business_planner::api::session::{Session, create_session, load_session};
use clap::{Arg, ArgAction, Command};

pub mod error;
pub mod plugins;
pub mod registry;
pub mod save;
pub mod utils;
pub mod graphs;

use enum_map::{Enum, EnumMap, enum_map};
pub use error::{Error, NonError};
use strum_macros::{Display, EnumString};

use crate::{
    plugins::PluginsMenu,
    registry::RegistryMenu,
    save::SaveMenu,
    utils::{Menu, PlannerResult, Submenu, get_command_matches},
};

fn entry_cli() -> Command {
    Command::new("business-planner-cli")
        .arg(
            Arg::new("interactive")
                .global(true)
                .required(false)
                .long("interactive")
                .action(ArgAction::SetTrue),
        )
        .subcommands([
            Command::new("create"),
            Command::new("load").arg(
                Arg::new("path")
                    .required(true)
                    .value_parser(clap::value_parser!(PathBuf)),
            ),
        ])
}

#[tokio::main]
async fn main() {
    let matches = entry_cli().get_matches();
    let mut session = match matches.subcommand() {
        Some(("load", arg_matches)) => {
            let path = arg_matches
                .get_one::<PathBuf>("path")
                .expect("Clap to have filtered off missing path argument");
            load_session(path).unwrap()
        }
        _ => create_session(),
    };
    let is_interactive = matches.get_one::<bool>("interactive").unwrap_or(&false);
    let mut user_requested_exit = false;
    while !user_requested_exit {
        let command = MainMenu::get_command();
        let result = main_loop(*is_interactive, command, &mut session).await;

        match result {
            Ok(NonError::Exit) => {
                user_requested_exit = true;
            }
            Ok(NonError::Continue) => {}
            Err(Error::UserCancelled) => {}
            Err(error) => println!("{}", error),
        };
    }
}

async fn main_loop(is_interactive: bool, command: Command, session: &mut Session) -> PlannerResult {
    match is_interactive {
        true => MainMenu::interactive(session).await,
        false => {
            print!("> ");
            let _ = stdout().flush();

            let mut buffer = String::new();
            let _ = stdin().read_line(&mut buffer);
            let arg_matches = get_command_matches(&buffer, command)?;
            MainMenu::non_interactive(&arg_matches, session).await
        }
    }
}

#[derive(Debug, Display, Enum, EnumString)]
enum MainMenu {
    Plugins,
    Registry,
    Save,
    Exit,
}

impl Menu for MainMenu {
    fn get_command() -> Command {
        Command::new("")
            .no_binary_name(true)
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommands(Self::get_subcommands())
    }

    fn get_submenus() -> EnumMap<Self, Submenu> {
        enum_map! {
            Self::Plugins => PluginsMenu::get_submenu(),
            Self::Registry => RegistryMenu::get_submenu(),
            Self::Save => SaveMenu::get_submenu(),
            Self::Exit => ExitMenu::get_submenu(),
        }
    }
}

#[derive(Debug, Display, Enum, EnumString)]
enum ExitMenu {}

impl Menu for ExitMenu {
    fn get_command() -> Command {
        Command::new("exit")
    }

    async fn interactive(_session: &mut Session) -> PlannerResult {
        Ok(NonError::Exit)
    }

    async fn non_interactive(_arg_matches: &clap::ArgMatches, _session: &mut Session) -> PlannerResult {
        Ok(NonError::Exit)
    }
}

// #[derive(Debug, Enum, EnumString)]
// enum MainMenu {
//     Plugins,
// }

// #[derive(Debug, Enum, EnumString)]
// enum PluginsMenu {
// }

// impl Menu for PluginsMenu {
//     fn get_name() -> String {
//         "Plugins".to_string()
//     }

//     fn get_action() -> Action {
//         Action::Traverse
//     }

//     fn get_submenu_populator() -> EnumMap<Self, MenuPopulator> {
//         enum_map! {
//         }
//     }
// }

// impl Menu for MainMenu {
//     fn get_name() -> String {
//         "Main Menu".to_string()
//     }

//     fn get_action() -> Action {
//         Action::Traverse
//     }

//     fn get_submenu_populator() -> EnumMap<Self, MenuPopulator> {
//         enum_map! {
//             Self::Plugins => PluginsMenu::populate,
//         }
//     }
// }

// impl BusinessPlannerCommand for MainMenuCommands {
//     fn get_command() -> Command {
//         Command::new("")
//             .no_binary_name(true)
//             .subcommand_required(true)
//             .arg_required_else_help(true)
//             .subcommands(Self::get_subcommands().into_values())
//     }

//     fn get_subcommands() -> EnumMap<Self, Box<dyn BusinessPlannerCommand>> {
//         enum_map! {
//             Self::Plugins => Box::new(PluginsCommands),
//             Self::Registry => RegistryCommands::get_command(),
//             Self::Graphs => todo!(),
//             Self::Save => SaveCommand::get_command(),
//             Self::Exit => Command::new("exit"),
//         }
//     }

//     async fn parse_user_command(user_command: UserCommand<Self>, session: &mut Session) -> Result<NonError, Error> {
//         match user_command {
//             UserCommand::Interactive(command) => {
//                 match command {
//                     Self::Plugins => PluginsCommands::run(true, session).await,
//                     Self::Registry => RegistryCommands::run(true, session).await,
//                     Self::Graphs => todo!(),
//                     Self::Save => SaveCommand::run(true, session).await,
//                     Self::Exit => Ok(NonError::Exit),
//                 }
//             },
//             UserCommand::NonInteractive(command, arg_matches) => {
//                 let (subcommand, subcommand_arg_matches) = arg_matches.subcommand().ok_or(Error::InvalidInput)?;
//                 match command {
//                     Self::Plugins => PluginsCommands::parse_user_command(UserCommand::NonInteractive(PluginsCommands::from_str(subcommand)?, *subcommand_arg_matches), session).await,
//                     Self::Registry => RegistryCommands::parse_user_command(UserCommand::NonInteractive(RegistryCommands::from_str(subcommand)?, *subcommand_arg_matches), session).await,
//                     Self::Graphs => todo!(),
//                     Self::Save => SaveCommand::parse_non_interactive_command(subcommand_arg_matches, session),
//                     Self::Exit => Ok(NonError::Exit),

//                 }
//             }
//         }
//     }
// }
