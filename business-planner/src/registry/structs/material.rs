use std::{collections::HashMap, fmt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{io::error::ReadError, registry::{Registry, RegistryItem, RegistryItemData, RegistryItemInternals, RegistryItemObject}};

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, Debug, Default, Clone)]
pub struct Material {
    name: Option<String>
}

impl Material {
    pub fn new() -> Material {
        Material::default()
    }

    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_string());
    }
}

impl RegistryItemData for Material {}

impl RegistryItem for Material {
    fn list_names(registry: &Registry) -> Vec<(&Uuid, Option<&str>)> {
        registry.materials.iter().map(|(uuid, material)| {
            (uuid, material.get_name())
        }).collect()
    }
}

impl RegistryItemObject for Material {
    type RegistryItemData = Material;
    type RegistryItem = Material;
}

impl RegistryItemInternals for Material {
    fn get_item_registry(registry: &Registry) -> &HashMap<Uuid, Material> {
        &registry.materials
    }

    fn get_item_registry_mut(registry: &mut Registry) -> &mut HashMap<Uuid, Material> {
        &mut registry.materials
    }

    fn fetch_data(&self) -> Result<Self, ReadError> {
        // let data = HashMap::<String, Vec<Data>>::new();
        Ok(self.clone())
    }
}

impl fmt::Display for Material {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.get_name().unwrap_or("");
        write!(f, "Material\nname:{}", name)
    }
}