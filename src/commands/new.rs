use rusqlite::{Connection, Result};

use crate::cli::NewCommand;
use crate::db::add_note::add_note;

pub fn new(cmd: NewCommand, conn: &Connection) -> Result<()> {
    let rows = add_note(conn, &cmd.title, &cmd.content)?;

    if rows == 1 {
        println!("Note added.");
    } else {
        eprintln!("Error in adding note");
    }

    Ok(())
}