use rusqlite::{Connection, Result};

use crate::cli::NewCommand;

pub fn new(cmd: NewCommand, conn: &Connection) -> Result<()> {
    conn.execute(
        "INSERT INTO notes (note) VALUES (?1)",
        [&cmd.note],
    )?;

    Ok(())
}