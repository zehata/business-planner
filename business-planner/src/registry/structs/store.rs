use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::registry::{Registry, RegistryItem, structs::DataSource};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Store {
    pub usage_data_source: DataSource,
}

impl RegistryItem for Store {
    fn get_item_registry(app_registry: &mut Registry) -> &mut HashMap<Uuid, Store> {
        &mut app_registry.stores
    }
}