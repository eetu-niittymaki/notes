use libsql::Connection;

use crate::error::Result;

pub async fn delete(
    conn: &Connection,
    id: i64
) -> Result<u64> {
        Ok(conn.execute(
            "DELETE FROM users 
                WHERE id = ?1",
        [id],
        ).await?)
}