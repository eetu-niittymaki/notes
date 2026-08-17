use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use crate::utils::export::build_html::build_html;
use crate::utils::export::build_md::build_md;
use crate::utils::export::build_txt::build_txt;
use crate::utils::export::build_json::build_json;

use notes_core::error::Result;

use notes_core::models::note::Note;
use notes_core::models::tag::Tag;

pub fn export_text(
    filetype: &str, 
    notes: Vec<Note>,
    note_tags: HashMap<i64, Vec<Tag>>,
    outfile_path: PathBuf
) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&outfile_path)?;

    match filetype {
        "txt" => write!(file, "{}", build_txt(notes, note_tags))?,
        "md" => write!(file, "{}", build_md(notes, note_tags))?,
        "html" => write!(file, "{}", build_html(notes, note_tags))?,
        "json" => write!(file, "{}", build_json(notes, note_tags))?,
        _ => unreachable!(),
    }

    Ok(())
}