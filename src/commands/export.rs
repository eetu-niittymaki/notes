use std::path::PathBuf;

use rusqlite::{Connection, Result};

use crate::cli::ExportCommand;
use crate::db::get_all_notes::get_all_notes;
use crate::utils::export_image::export_image;
use crate::utils::export_text::export_text;

pub fn export(cmd: ExportCommand, conn: &Connection) -> Result<()> {
    const SUPPORTED_FILETYPES: [&str; 5] = ["md", "txt", "html", "png", "pdf"];

    if !SUPPORTED_FILETYPES.contains(&cmd.filetype.as_str()) {
        eprintln!(
            "Unsupported filetype, supported formats: {}",
            SUPPORTED_FILETYPES.join(", ")
        );
        std::process::exit(0);
    }

    let notes = get_all_notes(conn)?;

    if notes.is_empty() {
        eprintln!("No notes found to export");
        std::process::exit(0);
    }

    let mut outfile_path = PathBuf::from(
        std::env::var("USERPROFILE").expect("USERPROFILE is not set"),
    );
    outfile_path.push("Desktop");
    outfile_path.push(format!("notes.{}", cmd.filetype));

    match cmd.filetype.as_str() {
        "txt" | "md" | "html" => export_text(cmd.filetype, notes, outfile_path),
        "png" | "pdf" => export_image(cmd.filetype, notes, outfile_path),
        _ => unreachable!(),
    }

    println!("Notes exported to desktop!");

    Ok(())
}
