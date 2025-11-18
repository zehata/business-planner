use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::registry::structs::{material::Material, store::Store};

pub mod structs;

#[derive(Serialize, Deserialize, PartialEq, Default)]
pub struct Registry {
    materials: HashMap<Uuid, Material>,
    stores: HashMap<Uuid, Store>,
}

impl Registry {
    pub fn create<T: RegistryItem>(&mut self, item: T) {
        RegistryItem::create_in_registry(item, self);
    }

    pub fn update<T: RegistryItem>(&mut self, id: Uuid, item: T) {
        RegistryItem::update_in_registry(id, item, self);
    }
}

#[derive(Serialize, Deserialize, PartialEq, Default)]
pub struct ItemRegistry<T> {
    registry: HashMap<Uuid, T>
}

pub trait RegistryItem: Sized {
    fn get_item_registry(app_registry: &mut Registry) -> &mut HashMap<Uuid, Self> where Self: Sized;

    fn create_in_registry(value: Self, app_registry: &mut Registry){
        let item_registry = Self::get_item_registry(app_registry);
        let id = Uuid::new_v4();
        item_registry.insert(id, value);
    }

    fn read_in_registry(id: Uuid, app_registry: &mut Registry) {
        let item_registry = Self::get_item_registry(app_registry);
        item_registry.get(&id);
    }

    fn update_in_registry(id: Uuid, value: Self, app_registry: &mut Registry) {
        let item_registry = Self::get_item_registry(app_registry);
        item_registry.insert(id, value);
    }

    fn delete_in_registry(id: Uuid, app_registry: &mut Registry) {
        let item_registry = Self::get_item_registry(app_registry);
        item_registry.remove(&id);
    }
}