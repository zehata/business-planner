use business_planner::api::session::Session;
use clap::{ArgMatches, Command};
use enum_map::{EnumMap, enum_map};

use crate::{
    Error,
    utils::{Menu, PlannerResult, RegistryItemTypes, Submenu, TakesItemId},
};

mod material;
mod store;

use {material::UpdateMaterialMenu, store::UpdateStoreMenu};

pub struct UpdateMenu {}

impl Menu for UpdateMenu {
    type SubcommandsEnum = RegistryItemTypes;

    fn get_command() -> Command {
        Command::new("update")
            .takes_item_id_arg()
    }

    fn get_submenus() -> EnumMap<RegistryItemTypes, Submenu> {
        enum_map! {
            RegistryItemTypes::Material => UpdateMaterialMenu::get_submenu(),
            RegistryItemTypes::Store => UpdateStoreMenu::get_submenu(),
            RegistryItemTypes::Ingredient => todo!(),
        }
    }

    async fn non_interactive(arg_matches: &ArgMatches, session: &mut Session) -> PlannerResult {
        let item_type = arg_matches
            .get_one::<Self::SubcommandsEnum>("item_type")
            .ok_or(Error::InvalidInput)?.to_owned();
        let submenus = Self::get_submenus();
        let submenu = &submenus[item_type];
        submenu.non_interactive(arg_matches, session).await
    }
}
