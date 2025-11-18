use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::registry::{Registry, RegistryItem};

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Material {

}

impl RegistryItem for Material {
    fn get_item_registry(app_registry: &mut Registry) -> &mut HashMap<Uuid, Material> {
        &mut app_registry.materials
    }
}