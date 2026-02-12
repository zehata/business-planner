use std::{cmp::Ordering, str::FromStr};
use business_planner::api::{item::IngredientItem, session::Session};
use clap::{Arg, ArgMatches, builder::{StringValueParser, TypedValueParser}};
use uuid::Uuid;

use crate::Error;

static ID_ARG_ID: &str = "ingredient_id";
static ID_ARG_KEBAB: &str = "ingredient-id";
static NAME_ARG_ID: &str = "ingredient_name";
static NAME_ARG_KEBAB: &str = "ingredient-name";

pub struct IngredientArg {}

impl IngredientArg {
    pub fn with_alias() -> impl Iterator<Item = Arg> {
        [
            Arg::new(ID_ARG_ID)
                .long(ID_ARG_KEBAB)
                .alias("id")
                .required_unless_present(NAME_ARG_ID)
                .conflicts_with(NAME_ARG_ID)
                .value_parser(
                    StringValueParser::new()
                    .try_map(|string| {
                        Uuid::from_str(&string)
                    })
                ),
            Arg::new(NAME_ARG_ID)
                .long(NAME_ARG_KEBAB)
                .alias("name")
                .conflicts_with(ID_ARG_ID)
                .required_unless_present(ID_ARG_ID),
        ].into_iter()
    }

    // pub fn without_alias() -> impl Iterator<Item = Arg> {
    //     [
    //         Arg::new(ID_ARG_ID)
    //             .long(ID_ARG_KEBAB)
    //             .required_unless_present(NAME_ARG_ID)
    //             .conflicts_with(NAME_ARG_ID)
    //             .value_parser(
    //                 StringValueParser::new()
    //                 .try_map(|string| {
    //                     Uuid::from_str(&string)
    //                 })
    //             ),
    //         Arg::new(NAME_ARG_ID)
    //             .long(NAME_ARG_KEBAB)
    //             .conflicts_with(ID_ARG_ID)
    //             .required_unless_present(ID_ARG_ID),
    //     ].into_iter()
    // }

    pub fn parse_arg_matches(arg_matches: &ArgMatches, session: &Session) -> Result<Uuid, Error> {
        if let Some(id) = arg_matches.get_one::<Uuid>(ID_ARG_ID) {
            return Ok(*id)
        }

        let name = arg_matches.get_one::<String>(NAME_ARG_ID).expect("");
        let mut candidates = session.get_by_name::<IngredientItem>(name);

        match (candidates.len()).cmp(&1) {
            Ordering::Less => Err(Error::NotFound(format!("ingredient with name {}", name))),
            Ordering::Equal => Ok(*candidates.remove(0)),
            Ordering::Greater => Err(Error::MultipleFound(format!("ingredient with name {}", name))),
        }
    }
}