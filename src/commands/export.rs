use std::path::PathBuf;

use rusqlite::{Connection, Result};

use crate::cli::ExportCommand;
use crate::db::get_all_notes::get_all_notes;
use crate::db::get_all_note_tags::get_all_note_tags;
use crate::utils::export::export_image::export_image;
use crate::utils::export::export_text::export_text;
use crate::utils::file_dialog::folder;

use crate::config::EXPORT_FILETYPES;

pub fn export(cmd: ExportCommand, conn: &Connection) -> Result<()> {
    if !EXPORT_FILETYPES.contains(&cmd.filetype.as_str()) {
        eprintln!(
            "Unsupported filetype, supported formats: {}",
            EXPORT_FILETYPES.join(", ")
        );
        std::process::exit(0);
    }

    let notes = get_all_notes(conn)?;

    if notes.is_empty() {
        eprintln!("No notes found to export");
        std::process::exit(0);
    }

    let note_tags = get_all_note_tags(conn)?;

    let mut outfile_path = PathBuf::new();
    
    let folder = match folder() {
        Some(path) => outfile_path.push(path),
        None => {
            println!("Folder selection cancelled");
            std::process::exit(0);
        }
    };

    outfile_path.push(format!("notes.{}", cmd.filetype));

    match cmd.filetype.as_str() {
        "txt" | "md" | "html" | "json" => export_text(cmd.filetype, notes, note_tags, outfile_path),
        "png" | "pdf" => export_image(cmd.filetype, notes, note_tags, outfile_path),
        _ => unreachable!(),
    }

    println!("Notes exported to {:?}", folder);

    Ok(())
}
