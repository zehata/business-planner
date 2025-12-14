use std::{path::Path};
use crate::io::error::ReadError;

pub fn read() -> Result<String, ReadError> {
    let path = Path::new("./src/io/samples/csv.csv");
    let mut sheet = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)?;
    let rows =sheet.records().collect::<Result<Vec<_>, _>>()?;
    let first_record = rows.first().ok_or(ReadError::NoRow)?;
    let row = first_record.iter().collect::<Vec<_>>();
    let value = row.get(1).ok_or(ReadError::NoCell)?;
    
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
