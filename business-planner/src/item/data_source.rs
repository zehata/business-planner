use std::{
    fmt::{self, Display},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ExcelDataSource {
    file_path: Option<PathBuf>,
    sheet: Option<String>,
    range: Option<String>,
}

impl ExcelDataSource {
    pub fn new(
        file_path: Option<&PathBuf>,
        sheet: Option<&str>,
        range: Option<&str>,
    ) -> ExcelDataSource {
        ExcelDataSource {
            file_path: file_path.cloned(),
            sheet: sheet.map(str::to_string),
            range: range.map(str::to_string),
        }
    }

    pub fn get_file_path(&self) -> Option<&PathBuf> {
        self.file_path.as_ref()
    }

    pub fn set_file_path(&mut self, file_path: Option<&PathBuf>) {
        self.file_path = file_path.cloned()
    }

    pub fn get_sheet(&self) -> Option<&str> {
        self.sheet.as_deref()
    }

    pub fn set_sheet(&mut self, sheet: Option<&str>) {
        self.sheet = sheet.map(str::to_string)
    }

    pub fn get_range(&self) -> Option<&str> {
        self.range.as_deref()
    }

    pub fn set_range(&mut self, range: Option<&str>) {
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
        writeln!(
            f,
            "Excel Data Source\nFile path: {}\nSheet: {}\nRange: {}",
            file_path, sheet, range
        )
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct PostgresqlDataSource {
    query: Option<String>,
}

impl PostgresqlDataSource {
    pub fn new(query: &str) -> PostgresqlDataSource {
        PostgresqlDataSource {
            query: Some(query.to_string()),
        }
    }

    pub fn get_query(&self) -> Option<&String> {
        self.query.as_ref()
    }

    pub fn set_query(&mut self, query: &str) {
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
        write!(
            f,
            "{}",
            match self {
                Self::Csv => "".to_string(),
                Self::Excel(excel_data_source) => format!("{}", excel_data_source),
                Self::Postgres(postgres_data_source) => format!("{}", postgres_data_source),
            }
        )
    }
}
