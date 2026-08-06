use rusqlite::{Connection, Result};

use crate::cli::SearchCommand;

pub fn search(cmd: SearchCommand, conn: &Connection) -> Result <()> {
    let pattern = format!("%{}%", cmd.content);

    let mut statement = conn.prepare(
        "SELECT * FROM notes
        WHERE note LIKE ?1",
    )?;

    let _ = statement.query([pattern])?;

    Ok(())
}