use std::path::PathBuf;

use bigdecimal::{BigDecimal};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

pub mod material;
pub mod store;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct ExcelDataSource {
    file_path: PathBuf,
    range: String,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum DataSource {
    Csv,
    Excel(ExcelDataSource),
    Postgres,
}

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Stock {
    material: material::Material,
    amount: Amount,
}
pub type Amount = BigDecimal;

#[derive(Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct StockLevel {
    pub amount: Amount,
    pub timestamp: Timestamp,
}

pub enum StockLevelTarget {
    TargetWindow{target: Amount, upward_window: Amount, downward_window: Amount},
    Thresholds{minimum: Amount, maximum: Amount},
}

#[derive(Serialize, Deserialize, PartialEq, Default)]
pub struct UsageData {
    pub stock_levels: Vec<StockLevel>
}