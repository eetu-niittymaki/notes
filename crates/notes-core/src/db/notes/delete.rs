use libsql::Connection;

use crate::error::Result;

pub async fn delete(
    conn: &Connection,
    user_id: i64,
    note_id: i64,
) -> Result<u64> {
        Ok(conn.execute(
            "DELETE FROM notes 
                WHERE id = ?1 
                AND user_id = ?2",
        [note_id, user_id],
        ).await?)
}

pub async fn delete_all(conn: &Connection, user_id: i64) -> Result<u64> {
    Ok(conn.execute(
        "DELETE FROM notes
            WHERE user_id = ?1",
            [user_id])
        .await?)
}