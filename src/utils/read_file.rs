use std::fs;
use std::path::Path;

pub fn read_file(file: &Path) -> String {
    let contents = fs::read_to_string(file)
        .expect("Error in reading imported file");

    contents
}