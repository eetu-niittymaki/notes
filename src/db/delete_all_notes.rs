use rusqlite::{Connection, Result};

pub fn delete_all_notes(conn: &Connection) -> Result<usize> {
    conn.execute(
        "DELETE FROM notes", 
        []
    ) 
}