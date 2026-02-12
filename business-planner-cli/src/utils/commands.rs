use std::{fmt::Display, pin::Pin};

use business_planner::api::session::Session;
use clap::{ArgMatches, Command};
use dialoguer::Completion;
use enum_map::{EnumArray, EnumMap};
use inquire::Select;
use strum::IntoEnumIterator;

use crate::{Error, NonError};

pub type PlannerResult = Result<NonError, Error>;
type PinnedPlannerResult<'a> = Pin<Box<dyn Future<Output = PlannerResult> + 'a>>;

pub struct Submenu {
    get_command: fn() -> Command,
    interactive: fn(&mut Session) -> PinnedPlannerResult,
    non_interactive: for<'a> fn(&'a ArgMatches, &'a mut Session) -> PinnedPlannerResult<'a>,
}

impl Default for Submenu {
    fn default() -> Self {
        unimplemented!()
    }
}

impl Submenu {
    pub fn get_command(&self) -> Command {
        (self.get_command)()
    }

    pub fn interactive<'a>(&self, session: &'a mut Session) -> PinnedPlannerResult<'a> {
        (self.interactive)(session)
    }

    pub fn non_interactive<'a>(
        &self,
        arg_matches: &'a ArgMatches,
        session: &'a mut Session,
    ) -> PinnedPlannerResult<'a> {
        (self.non_interactive)(arg_matches, session)
    }
}

pub trait Menu {
    type SubcommandsEnum: Display + EnumArray<Submenu> + IntoEnumIterator;

    fn get_submenu() -> Submenu {
        Submenu {
            get_command: Self::get_command,
            interactive: Self::pinned_interactive,
            non_interactive: Self::pinned_non_interactive,
        }
    }

    fn get_command() -> Command;

    fn get_submenus() -> EnumMap<Self::SubcommandsEnum, Submenu> {
        EnumMap::from_iter([])
    }

    fn get_subcommands() -> impl Iterator<Item = Command> {
        Self::get_submenus()
            .into_values()
            .map(|submenu| (submenu.get_command)())
    }

    fn pinned_interactive<'a>(session: &'a mut Session) -> PinnedPlannerResult<'a> {
        Box::pin(async move { Self::interactive(session).await })
    }

    fn pinned_non_interactive<'a>(
        arg_matches: &'a ArgMatches,
        session: &'a mut Session,
    ) -> PinnedPlannerResult<'a> {
        Box::pin(async move { Self::non_interactive(arg_matches, session).await })
    }

    #[allow(async_fn_in_trait)]
    async fn interactive(session: &mut Session) -> PlannerResult {
        let submenus = <Self::SubcommandsEnum>::iter();
        let selected_submenu = prompt_select_command(submenus)?;
        let submenu = &Self::get_submenus()[selected_submenu];
        submenu.interactive(session).await
    }

    #[allow(async_fn_in_trait)]
    async fn non_interactive(arg_matches: &ArgMatches, session: &mut Session) -> PlannerResult {
        let (command, subcommand_arg_matches) = arg_matches.subcommand().expect("");
        let submenus = Self::get_submenus();
        let selected_submenu = submenus
            .into_values()
            .find(|submenu| submenu.get_command().get_name() == command)
            .ok_or(Error::InvalidInput)?;
        selected_submenu
            .non_interactive(subcommand_arg_matches, session)
            .await
    }
}

pub fn prompt_select_command<T: Iterator<Item = U> + Clone, U: Display>(mut commands: T) -> Result<U, Error> {
    let command_names = commands
        .clone()
        .map(|command| format!("{}", command))
        .collect();
    let selected_command = Select::new("Select", command_names)
        .raw_prompt_skippable()?
        .ok_or(Error::UserCancelled)?;
    Ok(commands.nth(selected_command.index)
        .expect("User cannot select if subcommands is empty"))
}

pub fn get_command_matches(input: &str, command: &Command) -> Result<ArgMatches, Error> {
    let args = shlex::split(input).ok_or(Error::ErroneousShlexInput)?;
    let arg_matches = command.to_owned().try_get_matches_from(args)?;
    Ok(arg_matches)
}

pub struct Completer<'a> {
    command: &'a Command
}

impl<'a> Completer<'a> {
    pub fn new(command: &'a Command) -> Self {
        Self {
            command
        }
    }
}

impl Completion for Completer<'_> {
    fn get(&self, input: &str) -> Option<String> {
        let args = shlex::split(input)?;
        let mut base_command = self.command;
        if args.len() > 1 {
            let traversal_result = args[..(args.len()-1)].iter().try_fold(self.command, |current_command, subcommand_str| {
                current_command.get_subcommands().find(|subcommand| {
                    subcommand.get_name() == subcommand_str
                }).ok_or(Error::InvalidInput)
            });
            match traversal_result {
                Ok(command) => {
                    base_command = command
                },
                Err(_) => return None,
            }
        }
        let incomplete_subcommand = args.last()?;
        let mut candidates = base_command.get_subcommands().filter_map(|subcommand| {
            let subcommand_name = subcommand.get_name();
            if subcommand_name.starts_with(incomplete_subcommand) {
                return Some(subcommand_name[incomplete_subcommand.len()..].to_string())
            };
            let subcommand_alias = subcommand.get_all_aliases();
            let mut candidate_aliases = subcommand_alias.filter_map(|alias| {
                match alias.starts_with(incomplete_subcommand) {
                    true => {
                        Some(alias[incomplete_subcommand.len()..].to_string())
                    },
                    false => None,
                }
            }).collect::<Vec<_>>();
            if candidate_aliases.len() == 1{
                return Some(candidate_aliases.remove(0))
            }
            None
        }).collect::<Vec<_>>();
        if candidates.len() == 1 {
            let autocompletion = Some(candidates.remove(0))?;
            return Some(format!("{}{} ", input, autocompletion));
        };
        None
    }
}