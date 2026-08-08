use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use crate::utils::build_html::build_html;

use crate::models::Note;

pub fn export_text(filetype: String, notes: Vec<Note> , outfile_path: PathBuf) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(outfile_path)
        .unwrap();

    match filetype.as_str() {
        "txt" => {
            for note in notes {
                writeln!(file, "{}: {}", note.title, note.content).unwrap();
            }
        }

        "md" => {
            for note in notes {
                writeln!(file, "## {}: {}", note.title, note.content).unwrap();
            }
        }

        "html" => {
            write!(file, "{}", build_html(notes)).unwrap();
        }

        _ => unreachable!(),
    }
}