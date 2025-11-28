use serde::{Deserialize, Serialize};

use crate::{registry::{structs::DataSource}};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Store {
    pub name: Option<String>,
    pub usage_data_source: Option<DataSource>,
}

impl Store {
    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_string());
    }
}