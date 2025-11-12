use crate::io::error::ReadError;

pub fn read() -> Result<String, ReadError> {
    use std::path::Path;
    use umya_spreadsheet::{self as spreadsheet};

    let path = Path::new("./src/io/samples/excel.xlsx");
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
