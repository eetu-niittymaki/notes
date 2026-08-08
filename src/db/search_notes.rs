use rusqlite::{Connection, Result};

use crate::models::Note;

pub fn search_notes(
    conn: &Connection,
    title: Option<&str>,
    content: Option<&str>,
) -> Result<Vec<Note>> {
    if let Some(title) = title {
        // Searching by title
        let pattern = format!("%{}%", title);
        let mut statement = conn.prepare(
            "SELECT id, title, content, created_at
             FROM notes
             WHERE title LIKE ?1",
        )?;

        let notes = statement
            .query_map([pattern], |row| {
                Ok(Note {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content: row.get(2)?,
                    created_at: row.get(3)?
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        return Ok(notes);
    }
    // Searching by text content
    if let Some(content) = content {
        let pattern = format!("%{}%", content);
        let mut statement = conn.prepare(
            "SELECT id, title, content, created_at
             FROM notes
             WHERE content LIKE ?1",
        )?;

        let notes = statement
            .query_map([pattern], |row| {
                Ok(Note {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content: row.get(2)?,
                    created_at: row.get(3)?
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        return Ok(notes);
    }
    // Return empty vector if no notes found
    Ok(Vec::new())
}