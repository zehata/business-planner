use polars::error::PolarsError;
use calamine::XlsxError;

#[derive(Debug)]
pub enum ExcelError {
    InvalidRange,
    RangeEmpty,
    PolarsError(PolarsError),
    CalamineError(XlsxError),
}

impl From<XlsxError> for ExcelError {
    fn from(value: XlsxError) -> Self {
        ExcelError::CalamineError(value)
    }
}

impl From<PolarsError> for ExcelError {
    fn from(value: PolarsError) -> Self {
        ExcelError::PolarsError(value)
    }
}