use std::{path::Path};
use crate::io::error::ReadError;

pub fn read() -> Result<String, ReadError> {
    let path = Path::new("./src/io/samples/csv.csv");
    let mut sheet = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)?;
    let rows =sheet.records().collect::<Vec<_>>();
    let Some(Ok(first_record)) = rows.first() else {
        return Err(ReadError::NoRow)
    };
    let row = first_record.iter().collect::<Vec<_>>();
    let Some(value) = row.get(1) else {
        return Err(ReadError::NoCell)
    };
    
    Ok(value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_reader() {
        let result = read().unwrap();
        assert_eq!(result, "100");
    }
}
