use rusqlite::{Connection, Result};

use crate::models::Note;

pub fn search_notes(conn: &Connection, content: String) -> Result<Vec<Note>> {
    let pattern = format!("%{}%", content);

    let mut statement = conn.prepare(
    "SELECT id, title, content FROM notes
        WHERE content LIKE ?1",
    )?;

    let notes = statement
        .query_map([pattern], |row| {
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(notes)
}