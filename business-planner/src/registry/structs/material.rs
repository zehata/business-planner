use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, Debug, Default)]
pub struct Material {
    name: Option<String>
}

impl Material {
    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_string());
    }
}