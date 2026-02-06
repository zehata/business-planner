use crate::io::excel::error::ExcelError;
#[cfg(feature = "csv")]
use csv::Error as CsvError;
#[cfg(feature = "postgres")]
use sqlx::Error as SqlxError;

#[derive(Debug)]
pub enum IoError {
    ReadError(ReadError),
    WriteError,
}

impl From<ReadError> for IoError {
    fn from(value: ReadError) -> Self {
        IoError::ReadError(value)
    }
}

#[derive(Debug)]
pub enum ReadError {
    #[cfg(feature = "csv")]
    CsvError(CsvError),
    ExcelError(ExcelError),
    #[cfg(feature = "postgres")]
    SqlxError(SqlxError),
    NoRow,
    NoCell,
    NoDataSource,
}

impl From<ExcelError> for ReadError {
    fn from(value: ExcelError) -> Self {
        ReadError::ExcelError(value)
    }
}

#[cfg(feature = "csv")]
impl From<CsvError> for ReadError {
    fn from(value: CsvError) -> Self {
        ReadError::CsvError(value)
    }
}

#[cfg(feature = "postgres")]
impl From<SqlxError> for ReadError {
    fn from(value: SqlxError) -> Self {
        ReadError::SqlxError(value)
    }
}
