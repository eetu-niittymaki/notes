use libsql::Connection;

use crate::error::Result;

pub async fn one(
    conn: &Connection, 
    name: &str
) -> Result<u64> {
    Ok(conn.execute(
        "DELETE FROM tags WHERE name = ?1",
        [name],
    ).await?)
}