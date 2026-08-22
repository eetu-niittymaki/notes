use libsql::Connection;

use crate::error::Result;

pub async fn add(
    conn: &Connection,
    user_id: i64,
    note_id: i64,
    tag: &str,
) -> Result<u64> {
    // Make sure the note belongs to the authenticated user.
    let mut rows = conn
        .query(
            "SELECT id
             FROM notes
             WHERE id = ?1
             AND user_id = ?2",
            (note_id, user_id),
        )
        .await?;

    rows.next()
        .await?
        .ok_or(crate::error::Error::NoteNotFound)?;

    // Create the user's tag if it doesn't already exist.
    conn.execute(
        "INSERT INTO tags (user_id, name)
         VALUES (?1, ?2)
         ON CONFLICT(user_id, name) DO NOTHING",
        (user_id, tag),
    )
    .await?;

    // Get the tag ID belonging to this user.
    let mut rows = conn
        .query(
            "SELECT id
             FROM tags
             WHERE name = ?1
             AND user_id = ?2",
            (tag, user_id),
        )
        .await?;

    let row = rows
        .next()
        .await?
        .ok_or(crate::error::Error::NoteNotFound)?;

    let tag_id: i64 = row.get(0)?;

    // Associate the tag with the user's note.
    let rows_changed = conn
        .execute(
            "INSERT INTO note_tags (note_id, tag_id)
             VALUES (?1, ?2)
             ON CONFLICT(note_id, tag_id) DO NOTHING",
            (note_id, tag_id),
        )
        .await?;

    Ok(rows_changed)
}