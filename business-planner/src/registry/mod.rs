use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{registry::structs::{material::Material, store::Store}, session::Session};

pub mod structs;

#[derive(Serialize, Deserialize, PartialEq, Default)]
pub struct Registry {
    materials: HashMap<Uuid, Material>,
    stores: HashMap<Uuid, Store>,
}

pub(crate) trait GetItemRegistry {
    fn get_item_registry(session: &mut Session) -> &mut HashMap<Uuid, Self> where Self: Sized;
}

pub trait RegistryItem: GetItemRegistry + Sized {
    fn create_in_session(value: Self, session: &mut Session){
        let item_registry = Self::get_item_registry(session);
        let id = Uuid::new_v4();
        item_registry.insert(id, value);
    }

    fn read_in_session(id: Uuid, session: &mut Session) {
        let item_registry = Self::get_item_registry(session);
        item_registry.get(&id);
    }

    fn update_in_session(id: Uuid, value: Self, session: &mut Session) {
        let item_registry = Self::get_item_registry(session);
        item_registry.insert(id, value);
    }

    fn delete_in_session(id: Uuid, session: &mut Session) {
        let item_registry = Self::get_item_registry(session);
        item_registry.remove(&id);
    }
}