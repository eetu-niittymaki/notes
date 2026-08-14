use std::fs;
use std::path::Path;

pub fn read_file_content(file: &Path) -> String {
    let content = fs::read_to_string(file)
        .expect("Error in reading imported file");

    content
}