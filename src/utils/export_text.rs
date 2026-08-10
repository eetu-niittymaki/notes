use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use crate::utils::build_html::build_html;

use crate::models::Note;
use crate::models::Tag;

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
            for note in notes {
                writeln!(file, "{}", 
                    note.created_at.split_whitespace().next().unwrap()
                ).unwrap();
                writeln!(file, "{}", note.title).unwrap();

                if let Some(tags) = note_tags.get(&note.id) {
                    for tag in tags {
                        writeln!(file, "#{}", tag.name).unwrap();
                    }
                }

                writeln!(file, "{}", note.content).unwrap();
                writeln!(file).unwrap();
            }
        }

        "md" => {
            for note in notes {
                writeln!(file, "### {}", 
                    note.created_at.split_whitespace().next().unwrap()
                ).unwrap();
                writeln!(file, "# {}", note.title).unwrap();
                
                if let Some(tags) = note_tags.get(&note.id) {
                    for tag in tags {
                        writeln!(file, "* #{}", tag.name).unwrap();
                    }
                }

                writeln!(file, "## {}", note.content).unwrap();
                writeln!(file).unwrap();
            }
        }

        "html" => {
            write!(file, "{}", build_html(notes, note_tags)).unwrap();
        }

        _ => unreachable!(),
    }
}