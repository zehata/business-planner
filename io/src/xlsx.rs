use std::path::Path;
use umya_spreadsheet::{self as spreadsheet};
use crate::errors::ReadError;

pub fn read() -> Result<String, ReadError> {
    let path = Path::new("./samples/excel.xlsx");
    let book = spreadsheet::reader::xlsx::read(path)?;
    let sheet = book.get_sheet_by_name("Sheet1").unwrap();
    let Some(cell) = sheet.get_cell("B1") else {
        return Err(ReadError::NoCell)
    };
    let value = cell.get_value();
    Ok(value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xlsx_reader() {
        let result = read().unwrap();
        assert_eq!(result, "100");
    }
}
