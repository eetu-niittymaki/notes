use std::path::PathBuf;

use rusqlite::{Connection, Result};

use crate::cli::ImportCommand;
use crate::utils::file_dialog::file;
use crate::utils::read_file_content::read_file_content;
use crate::utils::import::md_to_text::md_to_text;
use crate::utils::import::html_to_text::html_to_text;
use crate::db::add_note::add_note;

use crate::config::IMPORT_FILETYPES;

pub fn import(cmd: ImportCommand, conn: &Connection) -> Result<()> {
    let file = match &cmd.file {
        Some(path) => PathBuf::from(path),
        None => file()
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

    match extension.as_deref() {
        Some("md") => {
            let text = md_to_text(&content);
            add_note(conn, title, &text).unwrap();
        },
        Some("html") => {
            let text = html_to_text(&content);
            add_note(conn, title, &text).unwrap();
        },
        Some("txt") => {
            add_note(conn, title, &content).unwrap();
        }
        _ => unreachable!()
    }

    println!("File {} imported!", format!("{}.{}", title, extension.unwrap()));

    Ok(())
}