use libsql::{Connection, params};

use crate::error::Result;

pub async fn create(
    conn: &Connection,
    user_id: i64,
    note_id: i64,
    version_number: i64,
    operation: &str,
    title: &str,
    content: &str,
) -> Result<u64> {
    Ok(conn.execute(
        "INSERT INTO notes_history (
            user_id, 
            note_id, 
            version_number,
            operation,
            title, 
            content
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![user_id, note_id, version_number, operation, title, content],
    ).await?)
}