use std::{collections::HashMap, fmt::Debug};

use uuid::Uuid;

mod material;
mod ingredient;
mod requirement;

mod store;
mod transfer;

use crate::{data::Data, io::error::ReadError, item::{ItemObject, NodeItem}};

pub use {material::MaterialResolver, ingredient::IngredientResolver, store::StoreResolver, transfer::TransferResolver, requirement::RequirementResolver};

#[derive(Debug, Default)]
pub struct Resolver {
    materials: HashMap<Uuid, MaterialResolver>,
    ingredients: HashMap<Uuid, IngredientResolver>,
    requirements: HashMap<Uuid, RequirementResolver>,

    stores: HashMap<Uuid, StoreResolver>,
    transfers: HashMap<Uuid, TransferResolver>,
}

impl Resolver {
    pub(crate) fn resolve<I: NodeItem>(&mut self, id: &Uuid, item: &I) -> Result<&<I::Resolver as ItemResolver>::Data, ReadError> {
        I::Resolver::resolve(self, id, item)
    }
}

pub trait ItemResolver: Debug {
    type ItemObject: ItemObject<Resolver = Self, Data = Self::Data>;
    type Data: Data;

    fn resolve<'a>(resolver: &'a mut Resolver, id: &Uuid, item: &Self::ItemObject) -> Result<&'a Self::Data, ReadError>;
}