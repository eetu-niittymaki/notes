use rusqlite::{Connection, Result};

use crate::models::Note;

pub fn notes(
    conn: &Connection,
    title: Option<&str>,
    content: Option<&str>,
) -> Result<Vec<Note>> {
    // Searching by title
    if let Some(title) = title {
        let pattern = format!("%{}%", title);
        let mut statement = conn.prepare(
            "SELECT id, 
                        title, 
                        content, 
                        created_at,
                        updated_at,
                        favorite
             FROM notes
             WHERE title LIKE ?1",
        )?;

        let notes = statement
            .query_map([pattern], |row| {
                Ok(Note {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                    favorite: row.get(5)?,
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        return Ok(notes);
    }
    
    // Searching by text content
    if let Some(content) = content {
        let pattern = format!("%{}%", content);
        let mut statement = conn.prepare(
            "SELECT id, 
                        title, 
                        content, 
                        created_at,
                        updated_at,
                        favorite
             FROM notes
             WHERE content LIKE ?1",
        )?;

        let notes = statement
            .query_map([pattern], |row| {
                Ok(Note {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                    favorite: row.get(5)?,
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        return Ok(notes);
    }
    // Return empty vector if no notes found
    Ok(Vec::new())
}

// Search for notes that have specific tag attached to them
pub fn tags(
    conn: &Connection, 
    tag: String,
) -> Result<Vec<Note>> {
    let mut statement = conn.prepare(
    "SELECT note.*
        FROM notes AS note
        JOIN note_tags AS note_tag ON note.id = note_tag.note_id
        JOIN tags AS tag ON note_tag.tag_id = tag.id
        WHERE tag.name = ?1"
    )?;

    let notes = statement
            .query_map([tag], |row| {
                Ok(Note {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                    favorite: row.get(5)?,
                })
            })?
            .collect::<Result<Vec<_>>>()?;

    return Ok(notes);
}