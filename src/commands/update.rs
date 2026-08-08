use rusqlite::{Connection, Result};

use crate::cli::UpdateCommand;
use crate::db::update_note::update_note;

pub fn update(cmd: UpdateCommand, conn: &Connection) -> Result<()> {
    let updated = update_note(conn, cmd.id, cmd.title.as_deref(), &cmd.new_content)?;

    if updated == 0 {
        println!("Note not found");
    } else {
        println!("Note updated");
    }

    Ok(())
}