mod ingredient_data;
mod material_data;
mod requirement_data;

mod store_data;
mod transfer_data;

use serde::{Deserialize, Serialize};
pub use {
    ingredient_data::IngredientData, material_data::MaterialData,
    requirement_data::RequirementData, store_data::StoreData, transfer_data::TransferData,
};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Value {
    Int64(i64),
    Float64(f64),
    String(String),
    Boolean(bool),
    Null,
}

pub trait Data: Serialize + Clone {}
