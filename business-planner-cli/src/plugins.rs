use clap::Command;
use enum_map::{Enum, enum_map};
use strum_macros::{Display, EnumString};

use crate::{
    plugins::{add::AddPluginMenu, remove::RemovePluginMenu, run::RunPluginMenu},
    utils::Menu,
};

mod add;
mod remove;
mod run;

#[derive(Debug, Display, Enum, EnumString)]
pub enum PluginsMenu {
    Run,
    Add,
    Remove,
}

impl Menu for PluginsMenu {
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
