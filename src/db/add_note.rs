use rusqlite::{Connection, Result};

pub fn add_note(
    conn: &Connection,
    title: &str,
    content: &str,
) -> Result<usize> {
    conn.execute(
        "INSERT INTO notes (title, content)
         VALUES (?1, ?2)",
        [title, content],
    )
}