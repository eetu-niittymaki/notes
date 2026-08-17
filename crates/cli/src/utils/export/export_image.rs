use std::path::PathBuf;
use std::collections::HashMap;

use hyper_render::{render, Config, OutputFormat};

use notes_core::error::Result;

use crate::utils::export::build_html::build_html;

use notes_core::models::note::Note;
use notes_core::models::tag::Tag;

pub fn export_image(
    filetype: &str, 
    notes: Vec<Note>, 
    note_tags: HashMap<i64, Vec<Tag>>, 
    outfile_path: PathBuf
) -> Result<()> {
    let html = build_html(notes, note_tags);

    let config = Config::default().format(match filetype {
        "png" => OutputFormat::Png,
        "pdf" => OutputFormat::Pdf,
        _ => unreachable!(),
    });

    let bytes = match render(&html, config) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Render failed: {e}");
            std::process::exit(0);
        }
    };

    if let Err(e) = std::fs::write(outfile_path, bytes) {
        eprintln!("Failed to write output: {e}");
        std::process::exit(0);
    }

    Ok(())
}
