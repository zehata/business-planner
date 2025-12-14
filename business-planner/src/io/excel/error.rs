use polars::error::PolarsError;

#[derive(Debug)]
pub enum ExcelError {
    InvalidRange,
    RangeEmpty,
    PolarsError(PolarsError),
}

impl From<PolarsError> for ExcelError {
    fn from(value: PolarsError) -> Self {
        ExcelError::PolarsError(value)
    }
}