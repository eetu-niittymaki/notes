use rusqlite::{Connection, Result};

use crate::cli::SearchCommand;
use crate::db::search_notes::search_notes;
use crate::db::search_tags::search_tags;

pub fn search(cmd: SearchCommand, conn: &Connection) -> Result<()> {
    if cmd.title.is_some() && cmd.content.is_some() {
        eprintln!("Please provide either a title or text content, not both.");
        return Ok(());
    }

    let notes = match (&cmd.title, &cmd.content, &cmd.tag) {
        (_, _, Some(tag)) => {
            search_tags(conn, tag.to_string())?
        }

        (Some(title), None, None) => {
            search_notes(conn, Some(title), None)?
        }

        (None, Some(content), None) => {
            search_notes(conn, None, Some(content))?
        }

        _ => {
            return Ok(());
        }
    };

    if notes.is_empty() {
        println!("No matching notes found");
        std::process::exit(0);
    }

    println!("Found Notes:");
    println!("-----------");

    for note in notes {
        println!("{} | {}: {}", note.id, note.title, note.content);
    }

    Ok(())
}