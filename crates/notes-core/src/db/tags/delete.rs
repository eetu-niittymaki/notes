use libsql::Connection;

use crate::error::Result;

pub async fn delete(
    conn: &Connection,
    user_id: i64, 
    name: &str
) -> Result<u64> {
    Ok(conn.execute(
        "DELETE FROM tags 
            WHERE name = ?1
            AND user_id = ?2",
        (name, user_id),
    ).await?)
}