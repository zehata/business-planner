use crate::io::error::ReadError;
use std::path::PathBuf;
use umya_spreadsheet::{self as spreadsheet, CellValue};

pub fn read_once(spreadsheet_path: &PathBuf, sheet: &str, range: &str) -> Result<Vec<CellValue>, ReadError> {
    let book = spreadsheet::reader::xlsx::read(spreadsheet_path)?;
    let sheet = book.get_sheet_by_name(sheet).unwrap();
    let values = sheet.get_cell_value_by_range(range);
    Ok(values.into_iter().cloned().collect())
}

#[cfg(test)]
mod tests {
    use umya_spreadsheet::CellRawValue;

    use super::*;

    #[test]
    fn test_xlsx_reader() {
        let result = read_once(&PathBuf::from("./samples/excel.xlsx"), "Sheet1", "B1:B3").unwrap();
        let cell_raw_values: Vec<_> = result.iter().map(|cell| {
            cell.get_raw_value()
        }).collect();
        assert_eq!(cell_raw_values, vec![&CellRawValue::Numeric(100.0), &CellRawValue::Numeric(90.0), &CellRawValue::Numeric(80.0)])
    }
}
