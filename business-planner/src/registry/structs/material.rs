use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{registry::{GetItemRegistry, RegistryItem}, session::Session};

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Material {

}

impl GetItemRegistry for Material {
    fn get_item_registry(session: &mut Session) -> &mut HashMap<Uuid, Material> {
        &mut session.data.registry.materials
    }
}

impl RegistryItem for Material {

}