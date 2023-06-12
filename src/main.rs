mod utils;

use ndarray::Array2;
use utils::mapCSV::*;

fn main() {
    let array = match parseCSV("map.csv") {
        Ok(array) => array,
        Err(err) => {
            println!("Error: {}", err);
            return;
        },
    };

    println!("{:?}", &array);
    println!("{}, {}", array.nrows(), array.ncols());
}