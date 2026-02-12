use clap::Command;
use enum_map::{EnumMap, enum_map};

use crate::utils::{ItemTypes, Menu, Submenu};

mod production_line;
mod store;
mod recipe;
mod ingredient;
mod material;

use {production_line::CreateProductionLineMenu, store::CreateStoreMenu, recipe::CreateRecipeMenu, ingredient::CreateIngredientMenu, material::CreateMaterialMenu};

pub struct CreateMenu {}

impl Menu for CreateMenu {
    type SubcommandsEnum = ItemTypes;

    fn get_command() -> clap::Command {
        Command::new("create")
            .subcommands(Self::get_subcommands())
    }

    fn get_submenus() -> EnumMap<ItemTypes, Submenu> {
        enum_map! {
            ItemTypes::ProductionLine => CreateProductionLineMenu::get_submenu(),
            ItemTypes::Store => CreateStoreMenu::get_submenu(),
            ItemTypes::Recipe => CreateRecipeMenu::get_submenu(),
            ItemTypes::Ingredient => CreateIngredientMenu::get_submenu(),
            ItemTypes::Material => CreateMaterialMenu::get_submenu(),
        }
    }
}