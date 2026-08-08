use rusqlite::{Connection, Result};

pub fn update_note(
    conn: &Connection,
    title: &str,
    content: &str,
) -> Result<usize> {
    conn.execute(
        "UPDATE notes
         SET content = ?1,
            updated_at = CURRENT_TIMESTAMP
         WHERE title = ?2",
        (content, title),
    )
}