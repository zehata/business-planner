use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{registry::{GetItemRegistry, RegistryItem, structs::DataSource}, session::Session};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Store {
    pub usage_data_source: Option<DataSource>,
}

impl GetItemRegistry for Store {
    fn get_item_registry(session: &mut Session) -> &mut HashMap<Uuid, Store> {
        &mut session.data.registry.stores
    }
}

impl RegistryItem for Store {

}