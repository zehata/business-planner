use std::{fmt::Display, pin::Pin};

use business_planner::api::session::Session;
use clap::{ArgMatches, Command};
use enum_map::{EnumArray, EnumMap};
use inquire::Select;

use crate::{Error, NonError};

pub type PlannerResult = Result<NonError, Error>;
pub type PinnedPlannerResult<'a> = Pin<Box<dyn Future<Output = PlannerResult> + 'a>>;

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

pub trait Menu: Display + EnumArray<Submenu> {
    fn get_submenu() -> Submenu {
        Submenu {
            get_command: Self::get_command,
            interactive: Self::pinned_interactive,
            non_interactive: Self::pinned_non_interactive,
        }
    }

    fn get_command() -> Command;

    fn get_submenus() -> EnumMap<Self, Submenu> {
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
        let submenus = Self::get_submenus();
        let mut options = submenus.iter();
        let options_names = options
            .clone()
            .map(|(command, _)| format!("{}", command))
            .collect();
        let selected_option = Select::new("Select", options_names)
            .raw_prompt_skippable()?
            .ok_or(Error::UserCancelled)?;
        let (_, submenu) = options
            .nth(selected_option.index)
            .expect("User cannot select if subcommands is empty");
        submenu.interactive(session).await
    }

    #[allow(async_fn_in_trait)]
    async fn non_interactive(arg_matches: &ArgMatches, session: &mut Session) -> PlannerResult {
        let (command, subcommand_arg_matches) = arg_matches.subcommand().expect("");
        let submenus = Self::get_submenus();
        let selected_submenu = submenus
            .into_values()
            .find(|submenu| (submenu.get_command)().get_name() == command)
            .ok_or(Error::InvalidInput)?;
        selected_submenu
            .non_interactive(subcommand_arg_matches, session)
            .await
    }
}

pub fn get_command_matches(buffer: &str, command: Command) -> Result<ArgMatches, Error> {
    let args = shlex::split(buffer).ok_or(Error::ErroneousShlexInput)?;
    let arg_matches = command.try_get_matches_from(args)?;
    Ok(arg_matches)
}
