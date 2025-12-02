use std::{collections::HashMap};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::registry::structs::{material::Material, store::Store};

pub mod structs;

#[derive(Serialize, Deserialize, PartialEq, Default)]
pub struct Registry {
    materials: HashMap<Uuid, Material>,
    stores: HashMap<Uuid, Store>,
}

pub enum RegistryItemType {
    Material,
    Store,
}

pub trait RegistryItem: Serialize {
    type Item: Default;

    fn get_item_registry(registry: &Registry) -> &HashMap<Uuid, Self::Item>;

    fn get_item_registry_mut(registry: &mut Registry) -> &mut HashMap<Uuid, Self::Item>;

    fn create(registry: &mut Registry, item: Self::Item) -> Uuid {
        let uuid = Uuid::new_v4();
        Self::get_item_registry_mut(registry).insert(uuid, item);
        uuid
    }

    fn read<'a>(id: &Uuid, registry: &'a mut Registry) -> Option<&'a mut Self::Item> {
        Self::get_item_registry_mut(registry).get_mut(id)
    }

    fn delete(id: &Uuid, registry: &mut Registry) -> Option<Self::Item>{
        Self::get_item_registry_mut(registry).remove(id)
    }

    fn list(registry: &mut Registry) -> Vec<String> {
        Self::get_item_registry(registry).keys().map(|key| {
            key.to_string()
        }).collect()
    }

    fn list_names(registry: &Registry) -> Vec<(&Uuid, Option<&str>)>;
}

impl Registry {
    pub fn create<T>(&mut self, item: T) -> Uuid
    where T: RegistryItem<Item = T> {
        T::create(self, item)
    }

    pub fn read<T>(&mut self, id: &Uuid) -> Option<&mut T> where T: RegistryItem<Item = T> {
        T::read(id, self)
    }

    pub fn delete<T>(&mut self, id: &Uuid) where T: RegistryItem<Item = T> {
        T::delete(id, self);
    }

    pub fn list<T>(&mut self) -> Vec<String> where T: RegistryItem<Item = T> {
        T::list(self)
    }

    pub fn list_names<T>(&mut self) -> Vec<(&Uuid, Option<&str>)> where T: RegistryItem<Item = T> {
        T::list_names(self)
    }
}