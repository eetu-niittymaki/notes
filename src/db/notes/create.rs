use rusqlite::{Connection, Result};

pub fn create(
    conn: &Connection,
    title: String,
    content: &str,
) -> Result<usize> {
    conn.execute(
        "INSERT INTO notes (title, content)
         VALUES (?1, ?2)",
        [title, content.to_string()],
    )
}