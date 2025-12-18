use std::{collections::HashMap, fmt::{self, Display}, path::PathBuf};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{io::{error::ReadError, excel::{create_vec_from_cells, read_once}}, registry::{Data, Registry, RegistryItem, RegistryItemData, RegistryItemInternals, RegistryItemObject}};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct StoreData {
    timestamps: Vec<Data>,
    // stock_levels: Vec<Data>,
}

impl StoreData {
    fn new(timestamps: Vec<Data>) -> StoreData {
        StoreData {
            timestamps,
        }
    }
}

impl RegistryItemData for StoreData {}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct Store {
    name: Option<String>,
    timestamps: Option<DataSource>,
    // stock_levels: Option<DataSource>,
}

impl Store {
    pub fn new() -> Store {
        Store::default()
    }

    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_string());
    }

    pub fn get_timestamps_range(&self) -> Option<&DataSource> {
        self.timestamps.as_ref()
    }

    pub fn get_timestamps_range_mut(&mut self) -> Option<&mut DataSource> {
        self.timestamps.as_mut()
    }

    pub fn set_timestamps_range(&mut self, data_source: Option<DataSource>) {
        self.timestamps = data_source;
    }
}

impl RegistryItem for Store {
    fn list_names(registry: &Registry) -> Vec<(&Uuid, Option<&str>)> {
        registry.stores.iter().map(|(uuid, store)| {
            (uuid, store.get_name())
        }).collect()
    }
}

impl RegistryItemObject for Store {
    type RegistryItemData = StoreData;
    type RegistryItem = Store;
}

impl RegistryItemInternals for Store {
    fn get_item_registry(registry: &Registry) -> &HashMap<Uuid, Self::RegistryItem> {
        &registry.stores
    }
    
    fn get_item_registry_mut(registry: &mut Registry) -> &mut HashMap<Uuid, Self::RegistryItem> {
        &mut registry.stores
    }

    fn fetch_data(&self) -> Result<StoreData, ReadError> {
        let timestamps = match &self.timestamps {
            Some(DataSource::Excel(excel_data_source)) => {
                let path = excel_data_source.get_file_path();
                let sheet = excel_data_source.get_sheet();
                let range = excel_data_source.get_range();

                match (path, sheet, range) {
                    (Some(path), Some(sheet), Some(range)) => {
                        let range = read_once(
                            path,
                            sheet,
                            range,
                        )?;
                        
                        create_vec_from_cells(range.cells())?
                    },
                    _ => {
                        vec![]
                    }
                }
            },
            Some(_) => todo!(),
            None => vec![],
        };

        let data = StoreData::new(timestamps);
        Ok(data)
    }
}

impl Display for Store {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name: &str = self.get_name().unwrap_or("");
        let timestamps_range = match self.get_timestamps_range() {
            Some(data_source) => {
                &format!("{}", data_source)
            },
            None => "",
        };
        write!(f, "Name: {}\nTimestamps:\n{}", name, timestamps_range)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ExcelDataSource {
    file_path: Option<PathBuf>,
    sheet: Option<String>,
    range: Option<String>,
}

impl ExcelDataSource {
    pub fn new(file_path: Option<&PathBuf>, sheet: Option<&str>, range: Option<&str>) -> ExcelDataSource {
        ExcelDataSource {
            file_path: file_path.cloned(),
            sheet: sheet.map(str::to_string),
            range: range.map(str::to_string),
        }
    }

    pub fn get_file_path (&self) -> Option<&PathBuf> {
        self.file_path.as_ref()
    }

    pub fn set_file_path (&mut self, file_path: Option<&PathBuf>) {
        self.file_path = file_path.cloned()
    }

    pub fn get_sheet (&self) -> Option<&str> {
        self.sheet.as_deref()
    }

    pub fn set_sheet (&mut self, sheet: Option<&str>) {
        self.sheet = sheet.map(str::to_string)
    }

    pub fn get_range (&self) -> Option<&str> {
        self.range.as_deref()
    }

    pub fn set_range (&mut self, range: Option<&str>) {
        self.range = range.map(str::to_string)
    }
}

impl Display for ExcelDataSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file_path = match self.get_file_path() {
            Some(file_path) => file_path.to_str().unwrap_or(""),
            None => "",
        };
        let sheet = self.get_sheet().unwrap_or("");
        let range = self.get_range().unwrap_or("");
        writeln!(f, "Excel Data Source\nFile path: {}\nSheet: {}\nRange: {}", file_path, sheet, range)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct PostgresqlDataSource {
    query: Option<String>
}

impl PostgresqlDataSource {
    pub fn new(query: &str) -> PostgresqlDataSource {
        PostgresqlDataSource {
            query: Some(query.to_string())
        }
    }

    pub fn get_query (&self) -> Option<&String> {
        self.query.as_ref()
    }

    pub fn set_query (&mut self, query: &str) {
        self.query = Some(query.to_string())
    }
}

impl Display for PostgresqlDataSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "PostgreSQL Data Source\n")
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum DataSource {
    Csv,
    Excel(ExcelDataSource),
    Postgres(PostgresqlDataSource),
}

impl Display for DataSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Csv => "".to_string(),
            Self::Excel(excel_data_source) => format!("{}", excel_data_source),
            Self::Postgres(postgres_data_source) => format!("{}", postgres_data_source),
        })
    }
}