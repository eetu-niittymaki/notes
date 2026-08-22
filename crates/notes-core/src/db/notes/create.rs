use libsql::{Connection, params};

use crate::error::Result;

pub async fn create(
    conn: &Connection,
    title: &str,
    content: &str,
) -> Result<u64> {
    Ok(conn.execute(
        "INSERT INTO notes (user_id, title, content)
            VALUES (?1, ?2, ?3)",
        params![1, title, content],
    ).await?)
}