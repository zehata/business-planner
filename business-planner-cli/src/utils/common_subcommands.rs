use clap::{Arg, Command, ValueEnum};
use enum_map::Enum;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Clone, Debug, Display, Enum, EnumIter, ValueEnum)]
pub enum NoSubcommands {}

pub trait TakesItemId {
    fn takes_item_id_arg(self) -> Command;
}

impl TakesItemId for Command {
    fn takes_item_id_arg(self) -> Command {
        self.arg(
            Arg::new("id")
                .long("id")
                .required(true)
        )
    }
}

#[derive(Clone, Debug, Display, Enum, EnumIter, EnumString, ValueEnum)]
pub enum ItemTypes {
    #[strum(serialize = "production-line", serialize = "line")]
    ProductionLine,
    #[strum(serialize = "store")]
    Store,
    #[strum(serialize = "recipe")]
    Recipe,
    #[strum(serialize = "ingredient")]
    Ingredient,
    #[strum(serialize = "material")]
    Material,
}

#[derive(Clone, Debug, Display, Enum, EnumIter, EnumString, ValueEnum)]
pub enum NodeItemTypes {
    #[strum(serialize = "store")]
    Store,
    #[strum(serialize = "ingredient")]
    Ingredient,
}

#[derive(Clone, Debug, Display, Enum, EnumIter, EnumString, ValueEnum)]
pub enum RegistryItemTypes {
    #[strum(serialize = "store")]
    Store,
    #[strum(serialize = "ingredient")]
    Ingredient,
    #[strum(serialize = "material")]
    Material,
}