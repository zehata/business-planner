use crate::{io::excel::error::ExcelError, registry::Data};
use std::path::PathBuf;
use calamine::{Cells, Data as CalamineData, Range, Reader, Xlsx, open_workbook};
use regex::Regex;

pub mod error;

pub fn convert_column_str_to_index(column_str: &str) -> Result<u32, ExcelError> {
    let column_number = column_str.as_bytes().iter().enumerate().try_fold(0, |acc, (_, char)| {
        let num = *char as u32;
        
        if !(65..=90).contains(&num) {
            return Err(ExcelError::InvalidRange)
        }
        
        Ok(acc*26 + (num - 64))
    })?;
    Ok(column_number - 1)
}

pub fn read_once(spreadsheet_path: &PathBuf, sheet: &str, range: &str) -> Result<Range<CalamineData>, ExcelError> {
    let mut book: Xlsx<_> = open_workbook(spreadsheet_path).unwrap();
    let sheet: Range<CalamineData> = book.worksheet_range(sheet).unwrap();

    let re = Regex::new(r"(?:(?<sheet_name>\w+)(?:!)){0,1}(?<range_start_column>[A-Za-z]{1,})(?<range_start_row>\d{0,})(?::(?<range_end_column>[A-Za-z]{1,})(?<range_end_row>\d{0,})){0,1}").unwrap();
    let captures = re.captures(range).ok_or(ExcelError::InvalidRange)?;

    let range_start_column = match &captures["range_start_column"] {
        "" => None,
        range_start_column => Some(convert_column_str_to_index(range_start_column)?),
    };
    let range_start_row = match &captures["range_start_row"] {
        "" => None,
        range_start_row => Some(str::parse::<u32>(range_start_row).expect("Regex to only match digits")-1),
    };

    let range_end_column = match &captures["range_end_column"] {
        "" => None,
        range_end_column => Some(convert_column_str_to_index(range_end_column)?),
    };
    let range_end_row = match &captures["range_end_row"] {
        "" => None,
        range_end_row => Some(str::parse::<u32>(range_end_row).expect("Regex to only match digits")-1),
    };

    let range = match (range_start_column, range_start_row, range_end_column, range_end_row) {
        (None, None, _, _) => return Err(ExcelError::InvalidRange),
        (None, _, _, None) => return Err(ExcelError::InvalidRange),
        (_, None, None, _) => return Err(ExcelError::InvalidRange),
        (None, Some(_), Some(_), Some(_)) => return Err(ExcelError::InvalidRange),
        (Some(_), None, Some(_), Some(_)) => return Err(ExcelError::InvalidRange),
        (Some(range_start_column), Some(range_start_row), None, None) => {
            // B2 => single cell
            sheet.range((range_start_row, range_start_column), (range_start_row, range_start_column))
        },
        (Some(range_start_column), range_start_row, Some(range_end_column), None) => {
            let range_start_row = range_start_row.unwrap_or(0);
            let range_end_row = sheet.end().ok_or(ExcelError::InvalidRange)?.0;
            let range = sheet.range((range_start_row, range_start_column), (range_end_row, range_end_column));
            let last_used_row = range.used_cells().next_back().ok_or(ExcelError::InvalidRange)?.0 as u32;
            sheet.range((range_start_row, range_start_column), (last_used_row, range_end_column))
        },
        (range_start_column, Some(range_start_row), None, Some(range_end_row)) => {
            let range_start_column = range_start_column.unwrap_or(0);
            let range_end_column = sheet.end().ok_or(ExcelError::InvalidRange)?.1;
            let range = sheet.range((range_start_row, range_start_column), (range_end_row, range_end_column));
            let last_used_column = range.used_cells().next_back().ok_or(ExcelError::InvalidRange)?.1 as u32;
            sheet.range((range_start_row, range_start_column), (range_end_row, last_used_column))
        },
        (Some(range_start_column), Some(range_start_row), Some(range_end_column), Some(range_end_row)) => {
            // B2:B4 => range from B2:N4
            sheet.range((range_start_row, range_start_column), (range_end_row, range_end_column))
        },
    };
    Ok(range)
}

pub fn create_vec_from_cells(cells: Cells<'_, CalamineData>) -> Result<Vec<Data>, ExcelError> {
    let values = cells.into_iter().map(|(_row, _column, data)| {
        match data {
            CalamineData::Int(value) => Data::Int64(*value),
            CalamineData::Float(value) => Data::Float64(*value),
            CalamineData::String(value) => Data::String(value.clone()),
            CalamineData::Bool(value) => Data::Boolean(*value),
            CalamineData::DateTime(value) => todo!(),
            CalamineData::DateTimeIso(value) => todo!(),
            CalamineData::DurationIso(_) => todo!(),
            CalamineData::Error(cell_error_type) => todo!(),
            CalamineData::Empty => Data::Null,
        }
    }).collect::<Vec<_>>();
    Ok(values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_parser() {
        let mut book: Xlsx<_> = open_workbook("./samples/offset_excel.xlsx").unwrap();
        let sheet = book.worksheet_range("Sheet1").unwrap();
        let mut range = sheet.range((0,2), (5,2));
        range = range.range((0, 2), (5, 2));
        println!("{}, {}", range.width(), range.height());
        println!("{:?}", range);
    }

    #[test]
    fn test_letter_to_index() {
        let index = convert_column_str_to_index("DF").unwrap();
        println!("{index}")
    }
}
