use std::str::FromStr;

use business_planner::api::session::Session;
use clap::{ArgMatches, Command};
use enum_map::{Enum, EnumMap, enum_map};
use strum_macros::{Display, EnumString};

use crate::{
    Error,
    registry::{
        TakesRegistryItemType,
        create::{
            ingredient::CreateIngredientMenu, material::CreateMaterialMenu, store::CreateStoreMenu,
        },
    },
    utils::{Menu, PlannerResult, Submenu},
};

pub mod ingredient;
pub mod material;
pub mod store;

#[derive(Debug, Display, Enum, EnumString)]
pub enum CreateRegistryItemMenu {
    Material,
    Store,
    Ingredient,
}

impl Menu for CreateRegistryItemMenu {
    fn get_command() -> Command {
        Command::new("create")
            .no_binary_name(true)
            .takes_registry_item_type_arg()
    }

    fn get_submenus() -> EnumMap<Self, Submenu> {
        enum_map! {
            Self::Material => CreateMaterialMenu::get_submenu(),
            Self::Store => CreateStoreMenu::get_submenu(),
            Self::Ingredient => CreateIngredientMenu::get_submenu(),
        }
    }

    async fn non_interactive(arg_matches: &ArgMatches, session: &mut Session) -> PlannerResult {
        let item_type = arg_matches
            .get_one::<String>("item_type")
            .ok_or(Error::InvalidInput)?;
        let item_type = CreateRegistryItemMenu::from_str(item_type)?;
        let submenus = Self::get_submenus();
        let submenu = &submenus[item_type];
        submenu.non_interactive(arg_matches, session).await
    }
}
