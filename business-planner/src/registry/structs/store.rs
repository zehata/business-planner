use std::{collections::HashMap, fmt::{self, Display}, path::PathBuf};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::registry::{Registry, RegistryItem};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Store {
    name: Option<String>,
    timestamps: Option<DataSource>,
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

    pub fn set_timestamps_range(&mut self, data_source: DataSource) {
        self.timestamps = Some(data_source);
    }
}

impl RegistryItem for Store {
    type Item = Store;

    fn get_item_registry(registry: &Registry) -> &HashMap<Uuid, Store> {
        &registry.stores
    }
    
    fn get_item_registry_mut(registry: &mut Registry) -> &mut HashMap<Uuid, Self::Item> {
        &mut registry.stores
    }

    fn list_names(registry: &Registry) -> Vec<(&Uuid, Option<&str>)> {
        registry.stores.iter().map(|(uuid, store)| {
            (uuid, store.get_name())
        }).collect()
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
        write!(f, "Store\nName: {}\nTimestamps:\n{}", name, timestamps_range)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct ExcelDataSource {
    file_path: Option<PathBuf>,
    range: Option<String>,
}

impl ExcelDataSource {
    pub fn new(file_path: PathBuf, range: &str) -> ExcelDataSource {
        ExcelDataSource {
            file_path: Some(file_path),
            range: Some(range.to_string()),
        }
    }

    pub fn get_file_path (&self) -> Option<&PathBuf> {
        self.file_path.as_ref()
    }

    pub fn set_file_path (&mut self, file_path: &PathBuf) {
        self.file_path = Some(file_path.to_owned())
    }
}

impl Display for ExcelDataSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file_path = match self.get_file_path() {
            Some(file_path) => file_path.to_str().unwrap_or(""),
            None => "",
        };
        writeln!(f, "Excel Data Source\nFile path: {}", file_path)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
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

#[derive(Serialize, Deserialize, PartialEq, Debug)]
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