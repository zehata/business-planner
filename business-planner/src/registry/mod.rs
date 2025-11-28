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

pub enum RegistryItemType {
    Material,
    Store,
}

#[derive(Serialize)]
pub enum RegistryItem<'a> {
    Material(&'a mut Material),
    Store(&'a mut Store),
}

impl Registry {
    pub fn create(&mut self, item_type: RegistryItemType) {
        let id = Uuid::new_v4();
        match item_type {
            RegistryItemType::Material => {
                self.materials.insert(id, Material::default());
            },
            RegistryItemType::Store => {
                self.stores.insert(id, Store::default());
            }
        }
    }

    pub fn read<'a>(&'a mut self, item_type: RegistryItemType, id: Uuid) -> Option<RegistryItem<'a>> {
        match item_type {
            RegistryItemType::Material => {
                Some(RegistryItem::Material(self.materials.get_mut(&id)?))
            },
            RegistryItemType::Store => {
                Some(RegistryItem::Store(self.stores.get_mut(&id)?))
            },
        }
    }


    pub fn delete(&mut self, item_type: RegistryItemType, id: Uuid) {
        match item_type {
            RegistryItemType::Material => {
                self.materials.remove(&id);
            },
            RegistryItemType::Store => {
                self.stores.remove(&id);
            },
        }
    }
}