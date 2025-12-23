
use serde::{Deserialize, Serialize};

use crate::{data::{Data, Value}};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct StoreData {
    timestamps: Vec<Value>,
    // stock_levels: Vec<Data>,
}

impl StoreData {
    pub(crate) fn new(timestamps: Vec<Value>) -> StoreData {
        StoreData {
            timestamps,
        }
    }
}

impl Data for StoreData {}