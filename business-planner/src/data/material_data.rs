use serde::{Deserialize, Serialize};

use crate::data::Data;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MaterialData {}

impl Data for MaterialData {}
