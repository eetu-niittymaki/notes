use rusqlite::{Connection, Result};

use crate::cli::SearchCommand;
use crate::db::search_notes::search_notes;

pub fn search(cmd: SearchCommand, conn: &Connection) -> Result<()> {
    if cmd.title.is_some() && cmd.content.is_some() {
        eprintln!("Please provide either a title or text content, not both.");
        return Ok(());
    }

    if cmd.title.is_some() || cmd.content.is_some() {
        let notes = search_notes(conn, cmd.title.as_deref(), cmd.content.as_deref())?;

        if notes.is_empty() {
            println!("No matching notes found");
            std::process::exit(1);
        }
    
        println!("Found Notes:");
        println!("-----------");

        for note in notes {
            println!("{} | {}: {}", note.id, note.title, note.content);
        }
    }
    
    Ok(())
}