use rusqlite::{Connection, Result};

use crate::cli::DeleteCommand;

pub fn delete(cmd: DeleteCommand, conn: &Connection) -> Result<()> {
    conn.execute(
        "DELETE FROM notes WHERE id = ?1",
        [cmd.id],
    )?;

    Ok(())
}