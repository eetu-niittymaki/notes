use rusqlite::{Connection, Result};

use crate::models::Note;

pub fn search_tags(
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