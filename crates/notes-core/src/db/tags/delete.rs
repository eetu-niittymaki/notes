use libsql::Connection;

use crate::error::Result;

pub async fn delete(
    conn: &Connection, 
    name: &str
) -> Result<u64> {
    Ok(conn.execute(
        "DELETE FROM tags 
            WHERE name = ?1
            AND user_id = ?2",
        (name, 1),
    ).await?)
}