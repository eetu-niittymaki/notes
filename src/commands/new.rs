use rusqlite::{Connection, Result};

use crate::config::DB_NAME;
use crate::cli::NewCommand;

pub fn new(cmd: NewCommand) -> Result<()> {
    let conn = Connection::open(DB_NAME)?;

    conn.execute(
        "INSERT INTO notes (note), VALUES (?1)",
        [&cmd.note]
    )?;

    Ok(())
}