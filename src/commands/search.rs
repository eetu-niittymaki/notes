use rusqlite::{Connection, Result};

use crate::cli::SearchCommand;
use crate::db::search_notes::search_notes;

pub fn search(cmd: SearchCommand, conn: &Connection) -> Result<()> {
    let notes = search_notes(conn, cmd.content)?;

    if notes.is_empty() {
        println!("No matching notes found");
        std::process::exit(1);
    }
    
    println!("Found notes");
    println!("-----------");

    for note in notes {
        println!("{} | {}: {}", note.id, note.title, note.content);
    }

    Ok(())
}