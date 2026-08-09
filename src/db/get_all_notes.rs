use rusqlite::{Connection, Result};

use crate::models::Note;

pub fn get_all_notes(conn: &Connection) -> Result<Vec<Note>> {
    let mut statement = conn.prepare(
        "SELECT id, 
                    title, 
                    content, 
                    created_at,
                    updated_at,
                    favorite
            FROM notes"
    )?;

    let notes = statement
        .query_map([], |row| {
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
                favorite: row.get(5)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(notes)
}