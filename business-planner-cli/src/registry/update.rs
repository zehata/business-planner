use std::str::FromStr;

use business_planner::api::session::Session;
use clap::{ArgMatches, Command};
use enum_map::{Enum, EnumMap, enum_map};
use strum_macros::{Display, EnumString};

use crate::{
    Error,
    registry::{
        TakesRegistryItemId, TakesRegistryItemType,
        update::{material::UpdateMaterialMenu, store::UpdateStoreMenu},
    },
    utils::{Menu, PlannerResult, Submenu},
};

mod material;
mod store;

#[derive(Debug, Display, Enum, EnumString)]
pub enum UpdateRegistryItemMenu {
    Material,
    Store,
    Ingredient,
}

impl Menu for UpdateRegistryItemMenu {
    fn get_command() -> Command {
        Command::new("update")
            .no_binary_name(true)
            .takes_registry_item_type_arg()
            .takes_registry_item_id_arg()
    }

    fn get_submenus() -> EnumMap<Self, Submenu> {
        enum_map! {
            Self::Material => UpdateMaterialMenu::get_submenu(),
            Self::Store => UpdateStoreMenu::get_submenu(),
            Self::Ingredient => todo!(),
        }
    }

    async fn non_interactive(arg_matches: &ArgMatches, session: &mut Session) -> PlannerResult {
        let item_type = arg_matches
            .get_one::<String>("item_type")
            .ok_or(Error::InvalidInput)?;
        let item_type = UpdateRegistryItemMenu::from_str(item_type)?;
        let submenus = Self::get_submenus();
        let submenu = &submenus[item_type];
        submenu.non_interactive(arg_matches, session).await
    }
}
