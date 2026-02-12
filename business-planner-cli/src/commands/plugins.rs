use clap::Command;
use enum_map::{Enum, enum_map};
use strum_macros::{Display, EnumIter, EnumString};

use crate::{utils::Menu};

mod add;
mod remove;
mod run;

use {add::AddPluginMenu, remove::RemovePluginMenu, run::RunPluginMenu};

#[derive(Debug, Display, Enum, EnumString, EnumIter)]
pub enum PluginsMenu {
    Run,
    Add,
    Remove,
}

impl Menu for PluginsMenu {
    type SubcommandsEnum = Self;

    fn get_command() -> Command {
        Command::new("plugins").subcommands(Self::get_subcommands())
    }

    fn get_submenus() -> enum_map::EnumMap<Self, crate::utils::Submenu> {
        enum_map! {
            Self::Run => RunPluginMenu::get_submenu(),
            Self::Add => AddPluginMenu::get_submenu(),
            Self::Remove => RemovePluginMenu::get_submenu(),
        }
    }
}
