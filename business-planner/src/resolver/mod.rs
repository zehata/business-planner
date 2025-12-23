use std::{collections::HashMap, fmt::Debug};

use uuid::Uuid;

mod material;
mod store;
use crate::{data::Data, io::error::ReadError, item::Item};

pub use {material::MaterialResolver, store::StoreResolver};

#[derive(Debug, Default)]
pub struct Resolver {
    materials: HashMap<Uuid, MaterialResolver>,
    stores: HashMap<Uuid, StoreResolver>,
}

impl Resolver {
    pub(crate) fn resolve<I: Item>(&mut self, id: &Uuid, registry_item: &I) -> Result<&<I::Resolver as ItemResolver>::Data, ReadError> {
        I::Resolver::resolve(self, id, registry_item)
    }
}

pub trait ItemResolver: Debug {
    type Item: Item<Resolver = Self, Data = Self::Data>;
    type Data: Data;

    fn resolve<'a>(resolver: &'a mut Resolver, id: &Uuid, registry_item: &Self::Item) -> Result<&'a Self::Data, ReadError>;
}