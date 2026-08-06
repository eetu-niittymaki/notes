use rusqlite::{Connection, Result};

use crate::cli::UpdateCommand;

pub fn update(cmd: UpdateCommand, conn: &Connection) -> Result<()> {
    conn.execute(
        "UPDATE notes
        SET note = ?1
        WHERE id = ?2",
        (&cmd.new_content, cmd.id),
    )?;

    Ok(())
}