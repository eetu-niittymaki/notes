use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use crate::utils::export::build_html::build_html;
use crate::utils::export::build_md::build_md;
use crate::utils::export::build_txt::build_txt;
use crate::utils::export::build_json::build_json;

use crate::models::{Note, Tag};

pub fn export_text(
    filetype: String, 
    notes: Vec<Note>,
    note_tags: HashMap<i64, Vec<Tag>>,
    outfile_path: PathBuf
) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(outfile_path)
        .unwrap();

    match filetype.as_str() {
        "txt" => {
            write!(file, "{}", build_txt(notes, note_tags)).unwrap();
        }

        "md" => {
            write!(file, "{}", build_md(notes, note_tags)).unwrap();
        }

        "html" => {
            write!(file, "{}", build_html(notes, note_tags)).unwrap();
        }

        "json" => {
            write!(file, "{}", build_json(notes, note_tags)).unwrap();
        }

        _ => unreachable!(),
    }
}