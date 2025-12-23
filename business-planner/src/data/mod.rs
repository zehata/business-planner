mod store_data;
mod material_data;

use {serde::{Deserialize, Serialize}};
pub use {store_data::StoreData, material_data::MaterialData};


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Value {
    Int64(i64),
    Float64(f64),
    String(String),
    Boolean(bool),
    Null,
}


pub trait Data: Serialize + Clone {}