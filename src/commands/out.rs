use std::path::PathBuf;

use rusqlite::{Connection, Result};

use crate::cli::OutCommand;
use crate::db::get_all_notes::get_all_notes;
use crate::utils::export_image::export_image;
use crate::utils::export_text::export_text;

pub fn out(cmd: OutCommand, conn: &Connection) -> Result<()> {
    let supported_filetypes = ["md", "txt", "html", "png", "pdf"];

    if !supported_filetypes.contains(&cmd.filetype.as_str()) {
        eprintln!(
            "Unsupported filetype, supported formats: {}",
            supported_filetypes.join(", ")
        );
        std::process::exit(1);
    }

    let notes = get_all_notes(conn)?;

    if notes.is_empty() {
        eprintln!("No notes found to export");
        std::process::exit(1);
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
