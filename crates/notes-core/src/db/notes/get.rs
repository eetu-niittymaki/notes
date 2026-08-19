use libsql::Connection;

use crate::error::Result;

use crate::models::note::{Note, NoteSelector, NoteWithTags};

use crate::db::tags::get::all_for_notes;

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

pub async fn all_with_tags(conn: &Connection) -> Result<Vec<NoteWithTags>> {
    let notes = all(conn).await?;
    let tags = all_for_notes(conn).await?;

    let notes = notes
        .into_iter()
        .map(|note| {
            let note_tags = tags
                .get(&note.id)
                .cloned()
                .unwrap_or_default();

            NoteWithTags {
                id: note.id,
                title: note.title,
                content: note.content,
                created_at: note.created_at,
                updated_at: note.updated_at,
                favorite: note.favorite,
                tags: note_tags,
            }
        })
        .collect();

    Ok(notes)
}