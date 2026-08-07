use rusqlite::{Connection, Result};

use crate::cli::NewCommand;

pub fn new(cmd: NewCommand, conn: &Connection) -> Result<()> {
    conn.execute(
        "INSERT INTO notes (title, content) 
            VALUES (?1, ?2)",
        [&cmd.title, &cmd.content],
    )?;

    Ok(())
}