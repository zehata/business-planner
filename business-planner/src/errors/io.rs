#[derive(Debug)]
pub enum ReadError {
    #[cfg(feature = "csv")]
    CsvError(CsvError),
    #[cfg(feature = "excel")]
    XlsxError(XlsxError),
    #[cfg(feature = "postgres")]
    SqlxError(SqlxError),
    NoRow,
    NoCell,
}

#[cfg(feature = "excel")]
use umya_spreadsheet::XlsxError;

#[cfg(feature = "excel")]
impl From<XlsxError> for ReadError {
    fn from(value: XlsxError) -> Self {
        ReadError::XlsxError(value)
    }
}

#[cfg(feature = "csv")]
use csv::{Error as CsvError};

#[cfg(feature = "csv")]
impl From<CsvError> for ReadError {
    fn from(value: CsvError) -> Self {
        ReadError::CsvError(value)
    }
}

#[cfg(feature = "postgres")]
use sqlx::{Error as SqlxError};

#[cfg(feature = "postgres")]
impl From<SqlxError> for ReadError {
    fn from(value: SqlxError) -> Self {
        ReadError::SqlxError(value)
    }
}