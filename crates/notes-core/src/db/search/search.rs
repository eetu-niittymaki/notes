use libsql::Connection;

use crate::error::Result;
use crate::models::note::Note;

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

pub async fn notes(
    conn: &Connection,
    user_id: i64,
    title: Option<&str>,
    content: Option<&str>,
) -> Result<Vec<Note>> {
    // Searching by title
    if let Some(title) = title {
        let pattern = format!("%{}%", title);

        let statement = conn
            .prepare(
                r#"
                SELECT id,
                       title,
                       content,
                       created_at,
                       updated_at,
                       favorite
                FROM notes
                WHERE title LIKE ?1
                AND user_id = ?2
                "#,
            )
            .await?;

        let mut rows = statement.query((pattern, user_id)).await?;

        let mut notes = Vec::new();

        while let Some(row) = rows.next().await? {
            notes.push(note_from_row(&row)?);
        }

        return Ok(notes);
    }

    // Searching by text content
    if let Some(content) = content {
        let pattern = format!("%{}%", content);

        let statement = conn
            .prepare(
                r#"
                SELECT id,
                       title,
                       content,
                       created_at,
                       updated_at,
                       favorite
                FROM notes
                WHERE content LIKE ?1
                AND user_id = ?2
                "#,
            )
            .await?;

        let mut rows = statement.query((pattern, user_id)).await?;

        let mut notes = Vec::new();

        while let Some(row) = rows.next().await? {
            notes.push(note_from_row(&row)?);
        }

        return Ok(notes);
    }

    Ok(Vec::new())
}

// Search for notes that have a specific tag attached to them
pub async fn tags(
    conn: &Connection,
    user_id: i64,
    tag: &str,
) -> Result<Vec<Note>> {
    let statement = conn
        .prepare(
            r#"
            SELECT note.id,
                   note.title,
                   note.content,
                   note.created_at,
                   note.updated_at,
                   note.favorite
            FROM notes AS note
            JOIN note_tags AS note_tag
                ON note.id = note_tag.note_id
            JOIN tags AS tag
                ON note_tag.tag_id = tag.id
            WHERE tag.name = ?1
            AND note.user_id = ?2
            AND tag.user_id = ?2
            "#,
        )
        .await?;

    let mut rows = statement.query((tag, user_id)).await?;

    let mut notes = Vec::new();

    while let Some(row) = rows.next().await? {
        notes.push(note_from_row(&row)?);
    }

    Ok(notes)
}