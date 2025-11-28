use std::path::PathBuf;
use serde::{Deserialize, Serialize};

pub mod material;
pub mod store;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ExcelDataSource {
    file_path: PathBuf,
    range: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum DataSource {
    Csv,
    Excel(ExcelDataSource),
    Postgres,
}