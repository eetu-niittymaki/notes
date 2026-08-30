use libsql::Connection;

use crate::error::Error::NotFound;
use crate::error::Result;

use crate::models::note::{Note, NoteWithTags};

use crate::db::tags::get::{all_for_notes, for_note};

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
    user_id: i64,
    note_id: i64,
) -> Result<NoteWithTags> {
    let mut rows = conn
        .query("
            SELECT id,
                    title,
                    content,
                    created_at,
                    updated_at,
                    favorite
            FROM notes
            WHERE id = ?1
            AND user_id = ?2",
            [note_id, user_id],
        )
        .await?;

    let row = match rows.next().await? {
        Some(row) => row,
        None => {
            return Err(NotFound.into());
        }
    };

    let tags = for_note(conn, user_id, note_id).await?;

    Ok(NoteWithTags {
        id: row.get(0)?,
        title: row.get(1)?,
        content: row.get(2)?,
        created_at: row.get(3)?,
        updated_at: row.get(4)?,
        favorite: row.get(5)?,
        tags,
    })
}

pub async fn all(
    conn: &Connection,
    user_id: i64
) -> Result<Vec<Note>> {
    let mut rows = conn
        .query(
            "SELECT id,
                    title,
                    content,
                    created_at,
                    updated_at,
                    favorite
             FROM notes
             WHERE user_id = ?1",
            [user_id]
        )
        .await?;

    let mut notes = Vec::new();

    while let Some(row) = rows.next().await? {
        notes.push(note_from_row(&row)?);
    }

    Ok(notes)
}

pub async fn all_with_tags(
    conn: &Connection,
    user_id: i64
) -> Result<Vec<NoteWithTags>> {
    let notes = all(conn, user_id).await?;
    let tags = all_for_notes(conn, user_id).await?;

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