
use rusqlite::{Connection, Result};

use crate::cli::DeleteCommand;

pub fn delete(cmd: DeleteCommand, conn: &Connection) -> Result<()> {
    if cmd.all {
        conn.execute("DELETE FROM notes", [])?;  
        println!("All notes deleted!");
    } else if cmd.title.is_some()  {
        conn.execute(
        "DELETE FROM notes WHERE title = ?1",
        [cmd.title],
        )?;
        println!("Note deleted!");
    } else {
        eprintln!("Please provide either an ID or -a/--all");
    }

    Ok(())
}