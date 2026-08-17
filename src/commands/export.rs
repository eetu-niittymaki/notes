use crate::error::Result;

use crate::cli::ExportCommand;

use crate::db::Database;

use crate::utils::export::export_image::export_image;
use crate::utils::export::export_text::export_text;
use crate::utils::file_dialog::folder;

use crate::config::EXPORT_FILETYPES;

pub fn export(cmd: ExportCommand, db: &Database,) -> Result<()> {
    if !EXPORT_FILETYPES.contains(&cmd.filetype.as_str()) {
        eprintln!(
            "Unsupported filetype, supported formats: {}",
            EXPORT_FILETYPES.join(", ")
        );
        std::process::exit(0);
    }

    let notes = db.notes().get_all()?;

    if notes.is_empty() {
        eprintln!("No notes found to export");
        std::process::exit(0);
    }

    let note_tags = db.tags().all_for_notes()?;

    let folder = match folder("Select Destination Folder") {
        Some(path) => path,
        None => {
            println!("Folder selection cancelled");
            std::process::exit(0);
        }
    };

    let outfile_path = folder.join(format!("notes.{}", cmd.filetype));

    match cmd.filetype.as_str() {
        "txt" | "md" | "html" | "json" => export_text(&cmd.filetype, notes, note_tags, outfile_path)?,
        "png" | "pdf" => export_image(&cmd.filetype, notes, note_tags, outfile_path)?,
        _ => unreachable!(),
    }

    println!("Notes exported to {:?}", folder);

    Ok(())
}
