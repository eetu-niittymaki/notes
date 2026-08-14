use std::{
    env::{self, var}, 
    fs, 
    process::Command,
};

pub fn editor(init_content: String, editor_name: Option<String>) -> String {
    let editor = var("EDITOR").unwrap_or_else(|_| "notepad.exe".to_string());

    let mut file_path = env::temp_dir();

    let editor_name = editor_name.unwrap_or("Edit".to_string());

    file_path.push(editor_name);

    fs::write(&file_path, init_content)
        .expect("Could not create temp file");

    Command::new(editor)
        .arg(&file_path)
        .status()
        .expect("Something went wrong");

     let edited_content = fs::read_to_string(&file_path)
        .expect("Could not read file");

    edited_content
}