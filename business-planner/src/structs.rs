use bigdecimal::{BigDecimal};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

use crate::usage_rates::PredictionError;

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Material {

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