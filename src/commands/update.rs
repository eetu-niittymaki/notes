use rusqlite::{Connection, Result};

use crate::cli::UpdateCommand;
use crate::db::update_note::update_note;

pub fn update(cmd: UpdateCommand, conn: &Connection) -> Result<()> {
    let updated = update_note(conn, &cmd.title, &cmd.new_content)?;

    if updated == 0 {
        println!("No note with title '{}' found.", cmd.title);
    } else {
        println!("Note updated.");
    }

    Ok(())
}