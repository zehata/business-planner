use clap::Command;
use enum_map::{Enum, enum_map};
use strum_macros::{Display, EnumIter, EnumString};

use crate::{
    graphs::{create::CreateGraphMenu, delete::DeleteGraphMenu, list::ListGraphsMenu, read::ReadGraphMenu, update::UpdateGraphMenu}, utils::Menu
};

mod create;
mod read;
mod update;
mod delete;
mod list;

#[derive(Debug, Display, Enum, EnumString)]
pub enum GraphsMenu {
    Create,
    Read,
    Update,
    Delete,
    List,
}

impl Menu for GraphsMenu {
    fn get_command() -> Command {
        Command::new("graphs").subcommands(Self::get_subcommands())
    }

    fn get_submenus() -> enum_map::EnumMap<Self, crate::utils::Submenu> {
        enum_map! {
            Self::Create => CreateGraphMenu::get_submenu(),
            Self::Read => ReadGraphMenu::get_submenu(),
            Self::Update => UpdateGraphMenu::get_submenu(),
            Self::Delete => DeleteGraphMenu::get_submenu(),
            Self::List => ListGraphsMenu::get_submenu(),
        }
    }
}

#[derive(Clone, Copy, Debug, Display, EnumIter, EnumString)]
pub enum GraphsType {
    ProductionLine,
    Recipe,
}