use rusqlite::{Connection, Result};

use crate::cli::AllCommand;
use crate::db::get_all_notes::get_all_notes;
use crate::db::get_all_note_tags::get_all_note_tags;

pub fn all(cmd: AllCommand, conn: &Connection) -> Result<()> {
    let notes = get_all_notes(conn)?;

    if notes.is_empty() {
        println!("No notes found");
        std::process::exit(0);
    }

    let note_tags = get_all_note_tags(conn)?;

    println!("All Notes:");
    println!("----------");

    for note in notes {
        println!("{} | {}", note.id, note.title);

        if let Some(tags) = note_tags.get(&note.id) {
            for tag in tags {
                println!("  #{}", tag.name);
            }
        }

        if cmd.content {
            println!("\n{}", note.content);
        }
    }

    Ok(())
}