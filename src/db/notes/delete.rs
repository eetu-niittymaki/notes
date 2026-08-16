use rusqlite::{Connection, Result};

use crate::models::NoteSelector;

pub fn one(
    conn: &Connection,
    selector: &NoteSelector
) -> Result<usize> {
    match selector {
        NoteSelector::Id(id) => {
            conn.execute(
                "DELETE FROM notes WHERE id = ?1",
            [id],
            )
        }

        NoteSelector::Title(title) => {
            conn.execute(
                "DELETE FROM notes WHERE title = ?1",
            [title],
            )
        }
    }
}

pub fn all(conn: &Connection) -> Result<usize> {
    conn.execute(
        "DELETE FROM notes", 
        []
    ) 
}