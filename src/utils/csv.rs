use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use ndarray::{Array2, s};

pub fn ParseCSV(path: &str) -> Result<Array2<(String, Option<i32>)>, Box<dyn Error>> {
    // Open the file
    let file = File::open(path)?;

    // Build CSV reader with ',' as delimiter and flexible number of fields
    let mut rdr = ReaderBuilder::new()
        .delimiter(b',')
        .flexible(true)
        .has_headers(false)  // Do not treat first row as headers
        .from_reader(file);

    let mut array_data = Vec::new();

    // Read each record
    for result in rdr.records() {
        let record = result?;

        // Skip empty rows
        if record.iter().next().is_none() {
            continue;
        }

        let mut row = Vec::new();
        for field in record.iter() {
            // Split the field into parts by '-'
            let parts: Vec<&str> = field.split('-').collect();
            // First part is a string and the second part is an integer (if present)
            let value = (
                parts[0].to_string(), 
                parts.get(1).and_then(|s| s.parse::<i32>().ok())
            );
            row.push(value);
        }

        array_data.push(row);
    }

    // Find maximum row length to handle jagged arrays
    let max_len = array_data.iter().map(|row| row.len()).max().unwrap_or(0);

    // Normalize rows to have equal length
    for row in &mut array_data {
        let diff = max_len - row.len();
        if diff > 0 {
            row.extend(vec![("".to_string(), None); diff]);
        }
    }

    // Convert the data to a 2D array
    let array: Array2<(String, Option<i32>)> = ndarray::Array::from_shape_vec((array_data.len(), max_len), array_data.concat())
        .expect("Error converting to 2D array");

    // Flip the array both horizontally and vertically
    let flipped_array = array.slice(s![..;-1, ..]).to_owned();

    Ok(flipped_array)
}

