use std::path::PathBuf;

use crate::error::Result;

use crate::models::cli::ImportCommand;

use crate::db::Database;

use crate::utils::file_dialog::file;
use crate::utils::read_file_content::read_file_content;
use crate::utils::import::import_without_separating::import_without_separating;
use crate::utils::import::import_with_separators::import_with_separators;
use crate::utils::get_user_input::get_user_input;

use crate::config::IMPORT_FILETYPES;

pub async fn import(cmd: ImportCommand, db: &Database,) -> Result<()> {
    // Get file through command line args or file dialog if no arg
    let file = match &cmd.file {
        Some(path) => PathBuf::from(path),
        None => {
            match file("Select File To Import") { // Get file through file dialog, print error if dialog exited
                Some(path) => path,
                None => {
                    println!("File selection canceled");
                    std::process::exit(0); 
                }
            }
        }
    };

    if !file.exists() {
        eprintln!("File not found!");
        std::process::exit(0);
    }

    let extension = file.extension().and_then(|ext| ext.to_str());

    if extension.is_none_or(|ext| !IMPORT_FILETYPES.contains(&ext)) {
        eprintln!(
            "Unsupported filetype, supported formats: {}",
            IMPORT_FILETYPES.join(", ")
        );
        std::process::exit(0);
    }

    let title = file.file_stem().unwrap().to_str().unwrap();
    let content = read_file_content(&file);

    println!("How should the file be imported? 
[1] Entire file as one note
[2] Manually add separator # at the title of each note. Rest of the text is considered content.");

    let mode: u8 = get_user_input()
        .parse()
        .unwrap_or(0);

    let result = match mode {
        1 => import_without_separating(db, &extension.unwrap(), content, title).await,
        2 => import_with_separators(db, &extension.unwrap(), content).await,
        _ => {
            println!("Please enter a number from 1-2.");
            return Ok(())
        }
    };

    println!("Successfully imported {} notes.", result.unwrap());

    Ok(())
}