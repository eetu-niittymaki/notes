use libsql::Connection;

use crate::error::Result;

pub async fn delete(
    conn: &Connection,
    id: i64
) -> Result<u64> {
        Ok(conn.execute(
            "DELETE FROM notes WHERE id = ?1",
        [id],
        ).await?)
}

pub async fn delete_all(conn: &Connection) -> Result<u64> {
    Ok(conn.execute(
        "DELETE FROM notes",())
        .await?)
}