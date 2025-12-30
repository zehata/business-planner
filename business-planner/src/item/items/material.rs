use std::{collections::{HashMap, HashSet}, fmt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{data::MaterialData, item::{Item, ItemAssociatedTypes, ItemObject, Registry, items::ItemInternals}, resolver::MaterialResolver, session::PersistentData};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MaterialItem {
    name: Option<String>,
    associated_ingredients: HashSet<Uuid>,
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

    pub fn add_associated_ingredient(&mut self, id: &Uuid) {
        self.associated_ingredients.insert(*id);
    }

    pub fn remove_associated_ingredient(&mut self, id: &Uuid) {
        self.associated_ingredients.remove(id);
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
        let material = registry.materials.remove(id);
        if let Some(material) = &material {
            let _ = material.associated_ingredients.iter().map(|id| {
                if let Some(ingredient) = registry.ingredients.get_mut(id) {
                    ingredient.set_material(None);
                }
            });
        };
        material
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
