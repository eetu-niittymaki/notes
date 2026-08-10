use rusqlite::{Connection, Result};

use crate::db::get_all_notes::get_all_notes;
use crate::db::get_tags_for_note::get_tags_for_note;

pub fn all(conn: &Connection) -> Result<()> {
    let notes = get_all_notes(conn)?;

    if notes.is_empty() {
        println!("No notes found");
        std::process::exit(0);
    }

    println!("All Notes:");
    println!("----------");

    for note in notes {
        let tags = get_tags_for_note(conn, note.id)?;

        println!("{} | {}", note.id, note.title);

        if !tags.is_empty() {
            for tag in tags {
                println!("  #{}", tag.name);
            }
            println!("\n{}", note.content);
        } else {
            println!("{}", note.content);
        }
    }

    Ok(())
}