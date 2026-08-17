use libsql::Connection;

use crate::error::Result;

use crate::models::{Note, NoteSelector};

fn note_from_row(row: &libsql::Row) -> Result<Note> {
    Ok(Note {
        id: row.get(0)?,
        title: row.get(1)?,
        content: row.get(2)?,
        created_at: row.get(3)?,
        updated_at: row.get(4)?,
        favorite: row.get(5)?,
    })
}

pub async fn one(
    conn: &Connection,
    selector: &NoteSelector<'_>,
) -> Result<Note> {
    let mut rows = match selector {
        NoteSelector::Id(id) => {
            conn.query(
                "SELECT id, title, content, created_at, updated_at, favorite
                 FROM notes
                 WHERE id = ?1",
                [*id],
            )
            .await?
        }

        NoteSelector::Title(title) => {
            conn.query(
                "SELECT id, title, content, created_at, updated_at, favorite
                 FROM notes
                 WHERE title = ?1",
                [*title],
            )
            .await?
        }
    };

    match rows.next().await? {
        Some(row) => note_from_row(&row),
        None => {
            // You need to decide what your application-level
            // "not found" error should be here.
            todo!("return your NoteNotFound error")
        }
    }
}

pub async fn all(conn: &Connection) -> Result<Vec<Note>> {
    let mut rows = conn
        .query(
            "SELECT id,
                    title,
                    content,
                    created_at,
                    updated_at,
                    favorite
             FROM notes",
            (),
        )
        .await?;

    let mut notes = Vec::new();

    while let Some(row) = rows.next().await? {
        notes.push(note_from_row(&row)?);
    }

    Ok(notes)
}