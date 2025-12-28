use serde::{Deserialize, Serialize};

use crate::data::Data;


#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct RequirementData {}

impl RequirementData {
    pub fn new() -> RequirementData {
        RequirementData {  }
    }
}

impl Data for RequirementData {}