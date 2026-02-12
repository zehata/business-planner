use clap::{Arg, Command, ValueEnum, builder::EnumValueParser};
use enum_map::Enum;
use strum_macros::{Display, EnumIter};

#[derive(Clone, Debug, Display, Enum, EnumIter, ValueEnum)]
pub enum ItemTypes {
    ProductionLine,
    Store,
    Recipe,
    Ingredient,
    Material,
}

pub trait TakesItemType {
    fn takes_item_type_arg(self) -> Command;
}

impl TakesItemType for Command {
    fn takes_item_type_arg(self) -> Command {
        self.arg(
            Arg::new("item_type")
                .required(true)
                .num_args(1)
                .value_parser(EnumValueParser::<ItemTypes>::new()),
        )
    }
}

pub trait TakesItemId {
    fn takes_item_id_arg(self) -> Command;
}

impl TakesItemId for Command {
    fn takes_item_id_arg(self) -> Command {
        self.arg(Arg::new("id").required(true))
    }
}