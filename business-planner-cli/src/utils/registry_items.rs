use clap::{Arg, Command, ValueEnum, builder::EnumValueParser};
use strum_macros::{Display, EnumIter};

#[derive(Clone, Debug, Display, EnumIter, ValueEnum)]
pub enum RegistryItemTypes {
    Material,
    Ingredient,
    Store,
}

pub trait TakesRegistryItemType {
    fn takes_registry_item_type_arg(self) -> Command;
}

impl TakesRegistryItemType for Command {
    fn takes_registry_item_type_arg(self) -> Command {
        self.arg(
            Arg::new("item_type")
                .required(true)
                .num_args(1)
                .value_parser(EnumValueParser::<RegistryItemTypes>::new()),
        )
    }
}