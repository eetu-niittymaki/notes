use rusqlite::{Connection, Result};

use crate::cli::NewCommand;

pub fn new(cmd: NewCommand, conn: &Connection) -> Result<()> {
    let note = cmd.note.join(" ");

    conn.execute(
        "INSERT INTO notes (note) VALUES (?1)",
        [&note],
    )?;

    Ok(())
}