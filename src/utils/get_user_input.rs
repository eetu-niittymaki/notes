use std::io::{self, Write};

pub fn get_user_input() -> String {
    let mut input = String::new();
    
    print!("");
    io::stdout().flush().expect("Failed to flush stdout");
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");


    input
}