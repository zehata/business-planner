use serde::{Deserialize, Serialize};

use crate::data::Data;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct IngredientData {}

impl Data for IngredientData {}
