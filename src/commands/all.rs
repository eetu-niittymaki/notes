use rusqlite::{Connection, Result};

use crate::db::get_all_notes::get_all_notes;

pub fn all(conn: &Connection) -> Result<()> {
    let notes = get_all_notes(conn)?;

    if notes.is_empty() {
        println!("No notes found");
        std::process::exit(0);
    }

    println!("All Notes:");
    println!("----------");

    for note in notes {
        println!("{} | {}\n{}", note.id, note.title, note.content);
    }

    Ok(())
}