
/*********************************** CSV ***********************************/

use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use ndarray::Array2;

pub fn parseCSV(path: &str) -> Result<Array2<i32>, Box<dyn Error>> {
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
            // Convert each field to an integer, using 0 if the field is empty
            let value: i32 = field.parse().unwrap_or(0);
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
            row.extend(vec![0; diff]);
        }
    }

    // Convert the data to a 2D array
    let mut array: Array2<i32> = ndarray::Array::from_shape_vec((array_data.len(), max_len), array_data.concat())
        .expect("Error converting to 2D array");

    // Adjust perimeter values to be 1 if they are 0
    let (max_i, max_j) = (array.nrows() - 1, array.ncols() - 1);
    for i in 0..=max_i {
        for j in 0..=max_j {
            if i == 0 || j == 0 || i == max_i || j == max_j {
                if array[[i, j]] == 0 {
                    array[[i, j]] = 1;
                }
            }
        }
    }

    Ok(array)
}