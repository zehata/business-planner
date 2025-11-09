use csv::{Error as CsvError};
use sqlx::{Error as SqlxError};
use umya_spreadsheet::XlsxError;

#[derive(Debug)]
pub enum ReadError {
    CsvError(CsvError),
    XlsxError(XlsxError),
    SqlxError(SqlxError),
    NoRow,
    NoCell,
}

impl From<XlsxError> for ReadError {
    fn from(value: XlsxError) -> Self {
        ReadError::XlsxError(value)
    }
}

impl From<CsvError> for ReadError {
    fn from(value: CsvError) -> Self {
        ReadError::CsvError(value)
    }
}

impl From<SqlxError> for ReadError {
    fn from(value: SqlxError) -> Self {
        ReadError::SqlxError(value)
    }
}