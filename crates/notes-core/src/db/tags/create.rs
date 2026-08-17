use libsql::Connection;

use crate::error::Result;

pub async fn add(
    conn: &Connection,
    id: i64,
    tag: &str,
) -> Result<u64> {
    // Create the tag if it doesn't already exist.
    conn.execute(
        "INSERT INTO tags (name)
         VALUES (?1)
         ON CONFLICT(name) DO NOTHING",
        [tag],
    )
    .await?;

    // Get the tag ID.
    let mut rows = conn
        .query(
            "SELECT id
             FROM tags
             WHERE name = ?1",
            [tag],
        )
        .await?;

    let row = rows
        .next()
        .await?
        .ok_or(crate::error::Error::NoteNotFound)?;

    let tag_id: i64 = row.get(0)?;

    // Associate the tag with the note.
    let rows_changed = conn
        .execute(
            "INSERT INTO note_tags (note_id, tag_id)
             VALUES (?1, ?2)
             ON CONFLICT(note_id, tag_id) DO NOTHING",
            [id, tag_id],
        )
        .await?;

    Ok(rows_changed)
}