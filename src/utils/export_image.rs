use std::path::PathBuf;

use hyper_render::{render, Config, OutputFormat};

use crate::utils::build_html::build_html;

use crate::models::Note;

pub fn export_image(filetype: String, notes: Vec<Note>, outfile_path: PathBuf) {
    let html = build_html(notes);

    let config = Config::default().format(match filetype.as_str() {
        "png" => OutputFormat::Png,
        "pdf" => OutputFormat::Pdf,
        _ => unreachable!(),
    });

    let bytes = match render(&html, config) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Render failed: {e}");
            std::process::exit(1);
        }
    };

    if let Err(e) = std::fs::write(outfile_path, bytes) {
        eprintln!("Failed to write output: {e}");
        std::process::exit(1);
    }
}
