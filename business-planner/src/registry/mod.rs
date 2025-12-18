use std::{collections::HashMap};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{api::registry::Store, io::error::ReadError, registry::structs::material::Material};

pub mod structs;

#[derive(Serialize, Deserialize, PartialEq, Default)]
pub struct Registry {
    materials: HashMap<Uuid, Material>,
    stores: HashMap<Uuid, Store>,
}

#[allow(private_bounds)]
pub trait RegistryItem: RegistryItemInternals {
    fn create(registry: &mut Registry, item: Self::RegistryItem) -> Uuid {
        let uuid = Uuid::new_v4();
        Self::get_item_registry_mut(registry).insert(uuid, item);
        uuid
    }

    fn read<'a>(id: &Uuid, registry: &'a Registry) -> Option<&'a Self::RegistryItem> {
        Self::get_item_registry(registry).get(id)
    }

    fn get<'a>(id: &Uuid, registry: &'a mut Registry) -> Option<&'a mut Self::RegistryItem> {
        Self::get_item_registry_mut(registry).get_mut(id)
    }

    fn update(id: &Uuid, registry: &mut Registry, item: Self::RegistryItem) {
        Self::get_item_registry_mut(registry).insert(*id, item);
    }

    fn delete(id: &Uuid, registry: &mut Registry) -> Option<Self::RegistryItem>{
        Self::get_item_registry_mut(registry).remove(id)
    }

    fn list(registry: &Registry) -> Vec<String> {
        Self::get_item_registry(registry).keys().map(|key| {
            key.to_string()
        }).collect()
    }

    fn list_names(registry: &Registry) -> Vec<(&Uuid, Option<&str>)>;
}

pub trait RegistryItemObject: Serialize + Default {
    type RegistryItem;
    type RegistryItemData: RegistryItemData;
}

pub(crate) trait RegistryItemInternals: RegistryItemObject {
    fn get_item_registry(registry: &Registry) -> &HashMap<Uuid, Self::RegistryItem>;

    fn get_item_registry_mut(registry: &mut Registry) -> &mut HashMap<Uuid, Self::RegistryItem>;

    fn fetch_data(&self) -> Result<Self::RegistryItemData, ReadError>;
}

impl Registry {
    pub fn create<T>(&mut self, item: T) -> Uuid
        where
            T: RegistryItem<RegistryItem = T> {
        T::create(self, item)
    }

    pub fn read<T>(&self, id: &Uuid) -> Option<&T> where T: RegistryItem<RegistryItem = T> {
        T::read(id, self)
    }

    pub fn get<T>(&mut self, id: &Uuid) -> Option<&mut T> where T: RegistryItem<RegistryItem = T> {
        T::get(id, self)
    }

    pub fn update<T>(&mut self, id: &Uuid, item: T)
    where T: RegistryItem<RegistryItem = T> {
        T::update(id, self, item)
    }

    pub fn delete<T>(&mut self, id: &Uuid) where T: RegistryItem<RegistryItem = T> {
        T::delete(id, self);
    }

    pub fn list<T>(&self) -> Vec<String> where T: RegistryItem<RegistryItem = T> {
        T::list(self)
    }

    pub fn list_names<T>(&self) -> Vec<(&Uuid, Option<&str>)> where T: RegistryItem<RegistryItem = T> {
        T::list_names(self)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Data {
    Int64(i64),
    Float64(f64),
    String(String),
    Boolean(bool),
    Null,
}

pub trait RegistryItemData: Serialize {}