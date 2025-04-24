use csv::Reader;
use serde::Deserialize;
use std::fs::File;

/// Record structure for CSV deserialization
#[derive(Debug, Deserialize)]
pub struct Record {
    pub value: String,
}

/// Read values from a CSV file
///
/// # Arguments
///
/// * `file_path` - Path to the CSV file
///
/// # Returns
///
/// A vector of strings containing the values from the CSV file
pub fn read_csv_values(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let mut reader = Reader::from_reader(file);
    let mut values = Vec::new();

    for result in reader.deserialize() {
        let record: Record = result?;
        values.push(record.value);
    }

    Ok(values)
}
