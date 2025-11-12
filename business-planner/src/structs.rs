use bigdecimal::{BigDecimal};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::{self, Display, Formatter}, fs, path::{Path, PathBuf}};

use crate::{session::error::SaveSessionError, usage_rates::PredictionError};

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Material {

}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct SessionData {
    schema_version: i32,
    materials: HashMap<String, Material>,
    stocks: HashMap<String, Stock>,
}

impl Default for SessionData {
    fn default() -> Self {
        SessionData {
            schema_version: 1,
            materials: HashMap::new(),
            stocks: HashMap::new(),
        }
    }
}

#[derive(Default)]
pub struct Session {
    pub last_save_location: Option<PathBuf>,
    pub data: SessionData,
}

impl Session {
    pub fn save_to_last_save_location(&self, overwrite: bool) -> Result<(), SaveSessionError> {
        match &self.last_save_location {
            Some(path) => self.save_to_location(path, overwrite),
            None => Err(SaveSessionError::UndefinedSavePath),
        }
    }

    pub fn save_to_location(&self, path: &Path, overwrite: bool) -> Result<(), SaveSessionError> {
        let serialized: String = serde_xml_rs::to_string(&self.data)?;
        if path.exists() && !overwrite {
            return Err(SaveSessionError::FileExists)
        }

        fs::write(path, serialized)?;
        
        Ok(())
    }
}

pub struct Store {
    pub usage_data: UsageData,
}

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Stock {
    material: Material,
    amount: Amount,
}
pub type Amount = BigDecimal;

#[derive(Clone)]
pub struct StockLevel {
    pub amount: Amount,
    pub timestamp: Timestamp,
}

pub enum StockLevelTarget {
    TargetWindow{target: Amount, upward_window: Amount, downward_window: Amount},
    Thresholds{minimum: Amount, maximum: Amount},
}

pub struct UsageData {
    pub stock_levels: Vec<StockLevel>
}

pub trait Predictor {
    fn time_at_minimum_threshold(&self, minimum_threshold: &Amount) -> Result<Timestamp, PredictionError>;

    fn display(&self) -> Box<dyn Display>;
}

impl fmt::Display for dyn Predictor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.display())
    }
}

pub type Report = String;