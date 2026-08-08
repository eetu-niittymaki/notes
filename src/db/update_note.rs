use rusqlite::{Connection, Result};

pub fn update_note(
    conn: &Connection,
    id: Option<i64>,
    title: Option<&str>,
    new_content: &str,
) -> Result<usize> {
    if let Some(id) = id {
        return conn.execute(
            "UPDATE notes
            SET content = ?1,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ?2",
            (new_content, id),
        );
    }

     if let Some(title) = title {
        return conn.execute(
            "UPDATE notes
            SET content = ?1,
                updated_at = CURRENT_TIMESTAMP
            WHERE title = ?2",
            (new_content, title),
        );
    }

    Ok(0)
}