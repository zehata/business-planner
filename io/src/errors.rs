use csv::{Error as CSVError};
use umya_spreadsheet::XlsxError;

#[derive(Debug)]
pub enum ReadError {
    CSVError(CSVError),
    XlsxError(XlsxError),
    NoRow,
    NoCell,
}

impl From<XlsxError> for ReadError {
    fn from(value: XlsxError) -> Self {
        ReadError::XlsxError(value)
    }
}

impl From<CSVError> for ReadError {
    fn from(value: CSVError) -> Self {
        ReadError::CSVError(value)
    }
}