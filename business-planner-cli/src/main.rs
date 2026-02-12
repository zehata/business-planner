use std::path::PathBuf;

use business_planner::api::session::{Session, create_session, load_session};
use clap::{Arg, ArgAction, Command, ValueEnum};

mod error;
mod commands;
mod utils;

use dialoguer::{BasicHistory, Input, console::style, theme::ColorfulTheme};
use enum_map::{Enum, EnumMap, enum_map};
pub use error::{Error, NonError};
use strum_macros::{Display, EnumIter, EnumString};

use crate::{
    // plugins::PluginsMenu,
    // registry::RegistryMenu,
    // save::SaveMenu,
    commands::{AddMenu, CreateMenu, DeleteMenu, ListMenu, PluginsMenu, ReadMenu, RemoveMenu, SaveMenu, UpdateMenu}, utils::{Completer, Menu, NoSubcommands, PlannerResult, Submenu, get_command_matches}
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
    match is_interactive {
        true => interactive(&mut session).await,
        false => non_interactive(&mut session).await,
    }
}

fn parse_result(planner_result: PlannerResult, user_requested_exit: &mut bool) {
    match planner_result {
        Ok(NonError::Exit) => {
            *user_requested_exit = true;
        }
        Ok(NonError::Continue) => {}
        Err(Error::UserCancelled) => {}
        Err(error) => println!("{}", error),
    };
}

async fn interactive(session: &mut Session) {
    let mut user_requested_exit = false;
    while !user_requested_exit {
        let planner_result = MainMenu::interactive(session).await;
        parse_result(planner_result, &mut user_requested_exit);
    }
}

async fn non_interactive(session: &mut Session) {
    let command = MainMenu::get_command();
    let completer = Completer::new(&command);
    let mut history = BasicHistory::new();

    let theme = ColorfulTheme {
        prompt_suffix: style(">".to_string()).for_stderr().black().bright(),
        ..Default::default()
    };

    let mut user_requested_exit = false;
    while !user_requested_exit {
        let input = Input::<String>::with_theme(&theme)
            .with_prompt("business-planner")
            .history_with(&mut history)
            .completion_with(&completer)
            .interact_text();

        let planner_result = match input {
            Ok(input) => non_interactive_loop(session, &command, input).await,
            Err(error) => Err(Error::DialoguerError(error)),
        };

        parse_result(planner_result, &mut user_requested_exit);
    }
}

async fn non_interactive_loop(session: &mut Session, command: &Command, line: String) -> PlannerResult {
    let arg_matches = get_command_matches(&line, command)?;
    MainMenu::non_interactive(&arg_matches, session).await
}

#[derive(Clone, Debug, Display, Enum, EnumIter, EnumString, ValueEnum)]
enum MainMenu {
    Create,
    Read,
    Delete,
    Update,
    List,
    Add,
    Remove,
    Plugins,
    Save,
    Exit,
}

impl Menu for MainMenu {
    type SubcommandsEnum = Self;

    fn get_command() -> Command {
        Command::new("")
            .no_binary_name(true)
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommands(Self::get_subcommands())
    }

    fn get_submenus() -> EnumMap<Self, Submenu> {
        enum_map! {
            Self::Create => CreateMenu::get_submenu(),
            Self::Read => ReadMenu::get_submenu(),
            Self::Update => UpdateMenu::get_submenu(),
            Self::Delete => DeleteMenu::get_submenu(),
            Self::List => ListMenu::get_submenu(),
            Self::Add => AddMenu::get_submenu(),
            Self::Remove => RemoveMenu::get_submenu(),
            Self::Plugins => PluginsMenu::get_submenu(),
            Self::Save => SaveMenu::get_submenu(),
            Self::Exit => ExitMenu::get_submenu(),
        }
    }
}

struct ExitMenu {}

impl Menu for ExitMenu {
    type SubcommandsEnum = NoSubcommands;

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
