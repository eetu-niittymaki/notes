use rusqlite::{Connection, Result};

pub fn add(
    conn: &Connection,
    id: i64, 
    tag: &str
) -> Result <usize> {
    let _ = conn.execute(
    "INSERT INTO tags (name)
         VALUES (?1)
         ON CONFLICT(name) DO NOTHING",
        [&tag],
    );

    let mut statement = conn.prepare(
    "SELECT id 
         FROM tags
         WHERE name = ?1"
    )?;

    let tag_id: i64 = statement.query_row(
        [&tag],
        |row| row.get(0),
    )?;

    let rows_changed = conn.execute(
    "INSERT INTO note_tags (note_id, tag_id)
        VALUES (?1, ?2)
        ON CONFLICT(note_id, tag_id) DO NOTHING",
        [id, tag_id]
    )?;

    Ok(rows_changed)
}