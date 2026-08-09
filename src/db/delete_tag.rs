use rusqlite::{Connection, Result};

pub fn delete_tag(
    conn: &Connection, 
    name: &str
) -> Result<usize> {
    return conn.execute(
        "DELETE FROM tags WHERE name = ?1",
        [name],
    );
}