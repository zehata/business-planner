use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{data::Data, resolver::ItemResolver};

mod edge_items;
mod items;

mod data_source;

pub use {
    data_source::{DataSource, ExcelDataSource, PostgresqlDataSource},
    edge_items::{EdgeItem, RequirementItem, TransferItem},
    items::{IngredientItem, Item, MaterialItem, NodeItem, StoreItem},
};

#[derive(Serialize, Deserialize, Default)]
pub struct Registry {
    materials: HashMap<Uuid, MaterialItem>,
    ingredients: HashMap<Uuid, IngredientItem>,
    requirements: HashMap<Uuid, RequirementItem>,

    stores: HashMap<Uuid, StoreItem>,
    transfers: HashMap<Uuid, TransferItem>,
}

pub trait ItemAssociatedTypes: Sized {
    type Data: Data;
    type Resolver: ItemResolver<ItemObject = Self, Data = Self::Data>;
}

pub(crate) trait ItemObject: ItemAssociatedTypes {
    fn get_item_registry(registry: &Registry) -> &HashMap<Uuid, Self>;

    fn get_item_registry_mut(registry: &mut Registry) -> &mut HashMap<Uuid, Self>;
}

impl Registry {
    pub fn create<T: Item>(&mut self, item: T) -> Uuid {
        T::create(item, self)
    }

    pub fn contains<T: Item>(&self, id: &Uuid) -> bool {
        T::contains(id, self)
    }

    pub fn read<T: Item>(&self, id: &Uuid) -> Option<&T> {
        T::read(id, self)
    }

    pub fn update<T: Item>(&mut self, id: &Uuid, item: T) {
        T::update(id, self, item)
    }

    pub fn list<T: Item>(&self) -> impl Iterator<Item = (&Uuid, Option<&str>)> {
        T::list(self)
    }
}
