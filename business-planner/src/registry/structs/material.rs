use std::{collections::HashMap, fmt};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::registry::{Registry, RegistryItem};

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, Debug, Default)]
pub struct Material {
    name: Option<String>
}

impl Material {
    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_string());
    }
}

impl RegistryItem for Material {
    type Item = Material;

    fn get_item_registry(registry: &Registry) -> &HashMap<Uuid, Material> {
        &registry.materials
    }

    fn get_item_registry_mut(registry: &mut Registry) -> &mut HashMap<Uuid, Material> {
        &mut registry.materials
    }

    fn list_names(registry: &Registry) -> Vec<(&Uuid, Option<&str>)> {
        registry.materials.iter().map(|(uuid, material)| {
            (uuid, material.get_name())
        }).collect()
    }
}

impl fmt::Display for Material {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.get_name().unwrap_or("");
        write!(f, "Material\nname:{}", name)
    }
}