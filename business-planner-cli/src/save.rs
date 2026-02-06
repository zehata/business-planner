use std::{path::PathBuf, str::FromStr};

use business_planner::api::session::{Session, save_to_last_save_location, save_to_location};
use clap::{Arg, ArgMatches, Command, value_parser};
use enum_map::{Enum, EnumMap, enum_map};
use inquire::Text;
use strum_macros::{Display, EnumString};

use crate::{
    Error, NonError,
    utils::{Menu, PlannerResult, Submenu},
};

#[derive(Debug, Display, Enum, EnumString)]
pub enum SaveMenu {
    Save,
    SaveAs,
}

impl Menu for SaveMenu {
    fn get_command() -> Command {
        Command::new("save").no_binary_name(true).arg(
            Arg::new("path")
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
    }

    fn get_submenus() -> EnumMap<Self, Submenu> {
        enum_map! {
            Self::Save => SaveToLastSaveLocationMenu::get_submenu(),
            Self::SaveAs => SaveAsMenu::get_submenu(),
        }
    }

    async fn non_interactive(arg_matches: &ArgMatches, session: &mut Session) -> PlannerResult {
        let path = arg_matches.get_one::<PathBuf>("path");

        match path {
            Some(path) => {
                save_to_location(session, path, true)?;
                Ok(NonError::Continue)
            }
            None => {
                save_to_last_save_location(session, true)?;
                Ok(NonError::Continue)
            }
        }
    }
}

#[derive(Debug, Display, Enum, EnumString)]
pub enum SaveToLastSaveLocationMenu {}

impl Menu for SaveToLastSaveLocationMenu {
    fn get_command() -> Command {
        unimplemented!()
    }

    async fn interactive(session: &mut Session) -> PlannerResult {
        save_to_last_save_location(session, true)?;
        Ok(NonError::Continue)
    }
}

#[derive(Debug, Display, Enum, EnumString)]
pub enum SaveAsMenu {}

impl Menu for SaveAsMenu {
    fn get_command() -> Command {
        unimplemented!()
    }

    async fn interactive(session: &mut Session) -> PlannerResult {
        let Ok(path) = Text::new("path").prompt() else {
            return Err(Error::UserCancelled);
        };
        let path = PathBuf::from_str(&path).expect("Pathbuf from String to be infallible");
        save_to_location(session, &path, true)?;
        Ok(NonError::Continue)
    }
}

// impl BusinessPlannerCommand for SaveMenu {

//     async fn parse_user_command(user_command: UserCommand<Self>, session: &mut Session) -> Result<NonError, Error> {
//         match user_command {
//             UserCommand::Interactive(command) => {
//                 match command {
//                     Self::Save => {

//                     },
//                     Self::SaveAs => {

//                     },
//                 }
//             },
//             UserCommand::NonInteractive(command, arg_matches) => unimplemented!(),
//         }
//     }
// }

// pub fn get_save_subcommand(is_interactive: bool) -> Command {
//     if is_interactive {
//         Command::new("save")
//     } else {
//         Command::new("save")
//             .no_binary_name(true)
//             .arg(
//                 Arg::new("path")
//                     .required(false)
//                     .value_parser(value_parser!(PathBuf))
//             )
//     }
// }

// pub fn get_save_interactive_subcommand() -> Vec<String> {
//     vec!["save".to_string(), "save as".to_string()]
// }

// pub fn parse_interactive_command<'a>(command: &'a str, session: &'a mut Session) -> PinnedResult<'a> {
//     Box::pin(async move {
//         match command {
//             "save" => {
//                 save_to_last_save_location(session, true)?;
//                 Ok(NonError::Continue)
//             },
//             "save as" => {
//                 let Ok(path) = Text::new("path").prompt() else {
//                     return Err(Error::UserCancelled)
//                 };
//                 let path = PathBuf::from_str(&path).expect("Pathbuf from String to be infallible");
//                 save_to_location(session, &path, true)?;
//                 Ok(NonError::Continue)
//             },
//             _ => Err(Error::InvalidInput),
//         }
//     })
// }

// pub fn parse_non_interactive_command<'a>(arg_matches: &'a ArgMatches, session: &'a mut Session) -> PinnedResult<'a> {
//     Box::pin(async move {
//         let path = arg_matches.get_one::<PathBuf>("path");

//         match path {
//             Some(path) => {
//                 save_to_location(session, path, true)?;
//                 Ok(NonError::Continue)
//             },
//             None => {
//                 save_to_last_save_location(session, true)?;
//                 Ok(NonError::Continue)
//             },
//         }
//     })
// }
