use std::io::{self, Write};

pub fn get_user_input() -> String {
    let mut input = String::new();

    let cursor = ">";
    print!("{} ", cursor);

    // Flush the output buffer
    io::stdout().flush().expect("Failed to flush stdout");
    
    // Read a line of input from standard input (keyboard)
    io::stdin()
    // Stores the input in variable
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}