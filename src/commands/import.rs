use std::path::Path;

use rusqlite::{Connection, Result};

use crate::cli::ImportCommand;
use crate::utils::read_file::read_file;
use crate::utils::md_to_text::md_to_text;
use crate::utils::html_to_text::html_to_text;
use crate::db::add_note::add_note;

pub fn import(cmd: ImportCommand, conn: &Connection) -> Result<()> {
    const SUPPORTED_FILETYPES: [&str; 3] = ["md", "txt", "html"];

    let file = Path::new(&cmd.file);

    if !file.exists() {
        eprintln!("File not found!");
        std::process::exit(0);
    }

    let extension = file.extension().and_then(|ext| ext.to_str());

    if extension.is_none_or(|ext| !SUPPORTED_FILETYPES.contains(&ext)) {
        eprintln!(
            "Unsupported filetype, supported formats: {}",
            SUPPORTED_FILETYPES.join(", ")
        );
        std::process::exit(0);
    }

    let title = file.file_stem().unwrap().to_str().unwrap();
    let content = read_file(file);

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

    Ok(())
}