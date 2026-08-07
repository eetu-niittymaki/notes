use rusqlite::{Connection, Result};

use crate::cli::UpdateCommand;

pub fn update(cmd: UpdateCommand, conn: &Connection) -> Result<()> {
    conn.execute(
        "UPDATE notes
        SET content = ?1,
            updated_at = CURRENT_TIMESTAMP
        WHERE title = ?2",
        (&cmd.new_content, cmd.title),
    )?;

    Ok(())
}