use libsql::{Connection, params};

use crate::error::Result;

pub async fn create(
    conn: &Connection,
    user_id: i64,
    note_id: i64,
    version_number: i64,
    title: &str,
    content: &str,
) -> Result<u64> {
    Ok(conn.execute(
        "INSERT INTO notes_history (
            user_id, 
            note_id, 
            version_number, 
            title, 
            content
        )
        VALUES (?1, ?2, ?3, ?4, ?5)",
        params![user_id, note_id, version_number, title, content],
    ).await?)
}