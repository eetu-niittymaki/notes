use libsql::{Connection, params};

use crate::error::Result;

pub async fn create(
    conn: &Connection,
    user_id: i64,
    title: &str,
    content: &str,
) -> Result<u64> {
    Ok(conn.execute(
        "INSERT INTO notes (user_id, title, content)
            VALUES (?1, ?2, ?3)",
        params![user_id, title, content],
    ).await?)
}