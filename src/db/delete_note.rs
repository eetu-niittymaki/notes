use rusqlite::{Connection, Result};

pub fn delete_note(
    conn: &Connection,
    id: Option<i64>,
    title: Option<&str>,
) -> Result<usize> {
    if let Some(id) = id {
        return conn.execute(
            "DELETE FROM notes WHERE id = ?1",
            [id],
        );
    }

    if let Some(title) = title {
        return conn.execute(
            "DELETE FROM notes WHERE title = ?1",
            [title],
        );
    }

    Ok(0)
}