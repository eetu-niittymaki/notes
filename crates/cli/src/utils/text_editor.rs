use std::{
    env::{self, var}, 
    fs, 
    process::Command,
};

pub fn text_editor(
    init_content: String, 
    editor_name: Option<String>
) -> String {
    // Get editor program from the `EDITOR` env variable
    // Default to notepad.exe if not found
    let editor = var("EDITOR").unwrap_or_else(|_| "notepad.exe".to_string());

    let mut temp_dir = env::temp_dir();

    let editor_name = editor_name.unwrap_or("Edit".to_string());

    temp_dir.push(editor_name);

    // Create and write original contents to temp file in temp directory
    fs::write(&temp_dir, init_content)
        .expect("Could not create temp file");

    // Open temp file in editor
    Command::new(editor)
        .arg(&temp_dir)
        .status()
        .expect("Something went wrong");

    let edited_content = fs::read_to_string(&temp_dir)
        .expect("Could not read file");

    edited_content
}