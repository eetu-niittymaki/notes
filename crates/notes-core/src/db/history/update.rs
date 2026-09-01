use libsql::Connection;

use crate::error::Result;

pub async fn restore_version(
    conn: &Connection,
    user_id: i64,
    note_id: i64,
    new_title: &str,
    new_content: &str
) -> Result<u64> {
    Ok(conn.execute(
    "UPDATE notes
            SET title = ?1,
                content = ?2,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ?3
            AND user_id = ?4",
        (new_title, new_content, note_id, user_id)
    ).await?)
}