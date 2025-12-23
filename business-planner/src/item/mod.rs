use std::{collections::HashMap};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{data::Data, resolver::ItemResolver};

mod material;
mod store;
mod data_source;
pub use {material::MaterialItem, store::StoreItem, data_source::{DataSource, ExcelDataSource, PostgresqlDataSource}};

#[derive(Serialize, Deserialize, PartialEq, Default)]
pub struct Registry {
    materials: HashMap<Uuid, MaterialItem>,
    stores: HashMap<Uuid, StoreItem>,
}

#[allow(private_bounds)]
pub trait Item: ItemIternals {
    fn create(registry: &mut Registry, item: Self) -> Uuid {
        let uuid = Uuid::new_v4();
        Self::get_item_registry_mut(registry).insert(uuid, item);
        uuid
    }

    fn read<'a>(id: &Uuid, registry: &'a Registry) -> Option<&'a Self> {
        Self::get_item_registry(registry).get(id)
    }

    fn get<'a>(id: &Uuid, registry: &'a mut Registry) -> Option<&'a mut Self> {
        Self::get_item_registry_mut(registry).get_mut(id)
    }

    fn update(id: &Uuid, registry: &mut Registry, item: Self) {
        Self::get_item_registry_mut(registry).insert(*id, item);
    }

    fn delete(id: &Uuid, registry: &mut Registry) -> Option<Self>{
        Self::get_item_registry_mut(registry).remove(id)
    }

    fn list(registry: &Registry) -> Vec<String> {
        Self::get_item_registry(registry).keys().map(|key| {
            key.to_string()
        }).collect()
    }

    fn list_names(registry: &Registry) -> Vec<(&Uuid, Option<&str>)>;
}

pub trait ItemObject: Sized {
    type Data: Data;
    type Resolver: ItemResolver<Item = Self, Data = Self::Data>;
}

pub(crate) trait ItemIternals: ItemObject {
    fn get_item_registry(registry: &Registry) -> &HashMap<Uuid, Self>;

    fn get_item_registry_mut(registry: &mut Registry) -> &mut HashMap<Uuid, Self>;
}

impl Registry {
    pub fn create<T: Item>(&mut self, item: T) -> Uuid {
        T::create(self, item)
    }

    pub fn read<T: Item>(&self, id: &Uuid) -> Option<&T> {
        T::read(id, self)
    }

    pub fn get<T: Item>(&mut self, id: &Uuid) -> Option<&mut T> {
        T::get(id, self)
    }

    pub fn update<T: Item>(&mut self, id: &Uuid, item: T) {
        T::update(id, self, item)
    }

    pub fn delete<T: Item>(&mut self, id: &Uuid) {
        T::delete(id, self);
    }

    pub fn list<T: Item>(&self) -> Vec<String> {
        T::list(self)
    }

    pub fn list_names<T: Item>(&self) -> Vec<(&Uuid, Option<&str>)> {
        T::list_names(self)
    }
}
