use std::{collections::HashMap, fmt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{data::MaterialData, item::{Item, ItemAssociatedTypes, ItemObject, Registry, items::ItemInternals}, resolver::MaterialResolver, session::PersistentData};

#[derive(Serialize, Deserialize, Hash, Debug, Default, Clone)]
pub struct MaterialItem {
    name: Option<String>
}

impl MaterialItem {
    pub fn new() -> MaterialItem {
        MaterialItem::default()
    }

    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_string());
    }
}

impl ItemAssociatedTypes for MaterialItem {
    type Data = MaterialData;
    type Resolver = MaterialResolver;
}

impl ItemObject for MaterialItem {
    fn get_item_registry(registry: &Registry) -> &HashMap<Uuid, Self> {
        &registry.materials
    }

    fn get_item_registry_mut(registry: &mut Registry) -> &mut HashMap<Uuid, Self> {
        &mut registry.materials
    }
}

impl ItemInternals for MaterialItem {}

impl Item for MaterialItem {
    fn delete(id: &Uuid, persistent_data: &mut PersistentData) -> Option<Self> {
        let registry = persistent_data.get_registry_mut();
        // registry.ingredients
        registry.materials.remove(id)
    }
    
    fn list_names(registry: &Registry) -> Vec<(&Uuid, Option<&str>)> {
        registry.materials.iter().map(|(uuid, material)| {
            (uuid, material.get_name())
        }).collect()
    }
}

impl fmt::Display for MaterialItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.get_name().unwrap_or("");
        write!(f, "Material\nname:{}", name)
    }
}
