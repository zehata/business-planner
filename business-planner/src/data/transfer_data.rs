use serde::{Deserialize, Serialize};

use crate::data::Data;


#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct TransferData {}

impl TransferData {
    pub fn new() -> TransferData {
        TransferData{}
    }
}

impl Data for TransferData {}