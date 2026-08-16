use rusqlite::{Connection, Result};

pub fn one(
    conn: &Connection, 
    name: &str
) -> Result<usize> {
    return conn.execute(
        "DELETE FROM tags WHERE name = ?1",
        [name],
    );
}