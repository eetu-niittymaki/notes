use libsql::Connection;

use crate::error::Result;

pub async fn create(
    conn: &Connection,
    title: &str,
    content: &str,
) -> Result<u64> {
    Ok(conn.execute(
        "INSERT INTO notes (title, content)
         VALUES (?1, ?2)",
        [title, content],
    ).await?)
}