use rusqlite::{Connection, Result};

use crate::models::{Note, NoteSelector};

pub fn get_note(
    conn: &Connection,
    selector: &NoteSelector
) -> Result<Note> {
    match selector {
        NoteSelector::Id(id) => {
            let mut statement = conn.prepare(
            "SELECT * FROM notes
                WHERE id = ?1"
            )?;

            let note: Note = statement.query_row(
        [id],
            |row| {
                Ok(Note {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                    favorite: row.get(5)?,
                })
            })?;
            Ok(note)
        }

        NoteSelector::Title(title) => {
            let mut statement = conn.prepare(
            "SELECT * FROM notes
                WHERE title = ?1"
            )?;

            let note: Note = statement.query_row(
            [title],
            |row| {
                Ok(Note {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                    favorite: row.get(5)?,
                })
            })?;
            Ok(note)
        }
    }
}