use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use notes_core::text_builders::build_html::build_html;
use notes_core::text_builders::build_md::build_md;
use notes_core::text_builders::build_txt::build_txt;
use notes_core::text_builders::build_json::build_json;

use notes_core::error::Result;
use notes_core::models::note::NoteWithTags;

pub fn export_text(
    filetype: &str, 
    notes: Vec<NoteWithTags>,
    outfile_path: PathBuf
) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&outfile_path)?;

    match filetype {
        "txt" => write!(file, "{}", build_txt(notes))?,
        "md" => write!(file, "{}", build_md(notes))?,
        "html" => write!(file, "{}", build_html(notes))?,
        "json" => write!(file, "{}", build_json(notes))?,
        _ => unreachable!(),
    }

    Ok(())
}